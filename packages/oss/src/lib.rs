//! Rusty API over Linux's OSS (/dev/dsp)

use bindings::audio_buf_info;
use nix::{fcntl::OFlag, libc};
use std::{
    fs::File,
    io::{self, Write},
    os::{fd::AsRawFd, unix::fs::OpenOptionsExt},
    path::Path, time::Instant,
};

mod bindings;

pub struct Device {
    file: File,
}

impl Device {
    pub fn new(path: impl AsRef<Path>, channels: i32, freq: i32) -> io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(false)
            .custom_flags(libc::O_NONBLOCK | libc::O_CLOEXEC)
            .open(path)?;

        // Set the device back to blocking IO after we open it
        let flags = nix::fcntl::fcntl(file.as_raw_fd(), nix::fcntl::FcntlArg::F_GETFL).unwrap();
        let blocking_flags = flags & !libc::O_NONBLOCK;
        nix::fcntl::fcntl(
            file.as_raw_fd(),
            nix::fcntl::FcntlArg::F_SETFL(OFlag::from_bits(blocking_flags).unwrap()),
        )
        .unwrap();

        let this = Self { file };

        let formats =
            unsafe { this.ioctl_read::<libc::c_int>(bindings::SNDCTL_DSP_GETFMTS) }.unwrap();

        if (formats & bindings::AFMT_S16_LE as i32) == 0 {
            panic!("Device must support signed 16-bit LE audio!");
        }

        let format = bindings::AFMT_S16_LE;
        unsafe { this.ioctl_write(bindings::SNDCTL_DSP_SETFMT, &format) }.unwrap();
        unsafe { this.ioctl_write(bindings::SNDCTL_DSP_CHANNELS, &channels) }.unwrap();
        unsafe { this.ioctl_write(bindings::SNDCTL_DSP_SPEED, &freq) }.unwrap();

        let buffer_size = 2048;
        let mut frag_spec = 0u32;

        while (0x01u32 << frag_spec) < buffer_size {
            frag_spec += 1;
        }

        frag_spec |= 0x00020000; // two fragments, for low latency

        tracing::info!(
            "Requesting {} fragments of size {}",
            (frag_spec >> 16),
            1 << (frag_spec & 0xFFFF)
        );

        unsafe { this.ioctl_write(bindings::SNDCTL_DSP_SETFRAGMENT, &frag_spec) }.unwrap();

        let info = this.info().unwrap();

        tracing::info!("Audio Info: {info:#?}");

        let mix_buf = vec![0; buffer_size as usize];

        Ok(this)
    }

    pub fn play(&mut self, data: &[i16]) -> io::Result<()> {
        let data_u8: &[u8] =
            unsafe { std::slice::from_raw_parts(data.as_ptr().cast(), data.len() * 2) };

        let start = Instant::now();
        self.file.write_all(data_u8)?;
        tracing::info!("Time to play {} bytes: {}ms", data_u8.len(), (Instant::now() - start).as_millis());

        Ok(())
    }

    pub fn play_until_empty(&mut self) -> io::Result<()> {
        let empty = 0;
        unsafe { self.ioctl_write(bindings::SNDCTL_DSP_SYNC, &empty) }
    }

    fn info(&self) -> io::Result<audio_buf_info> {
        unsafe { self.ioctl_read(bindings::SNDCTL_DSP_GETOSPACE) }
    }

    unsafe fn ioctl_read<T: Default>(&self, req: u32) -> io::Result<T> {
        let mut value = T::default();

        if unsafe { libc::ioctl(self.file.as_raw_fd(), req, &mut value) } < 0 {
            return Err(nix::Error::last().into());
        };

        Ok(value)
    }

    unsafe fn ioctl_write<T>(&self, req: u32, value: &T) -> io::Result<()> {
        if unsafe { libc::ioctl(self.file.as_raw_fd(), req, value) } < 0 {
            return Err(nix::Error::last().into());
        };

        Ok(())
    }
}

fn next_power_of_2(x: i32) -> i32 {
    let mut value;

    if x <= 0 {
        /* Return some sane value - we shouldn't hit this in our use cases */
        return 1;
    }

    /* This trick works for 32-bit values */
    value = x;
    value -= 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value += 1;

    value
}
