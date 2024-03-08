//! Thanks to OnionOS for most of this code
use std::{io, os::fd::AsRawFd};

use tokio::fs::OpenOptions;

pub const MIN_VOLUME: i32 = 0;
pub const MAX_VOLUME: i32 = 20;
const MIN_RAW_VALUE: i32 = -60;
const MAX_RAW_VALUE: i32 = 30;

const MI_AO_SETVOLUME: u32 = 0x4008690b;
const MI_AO_GETVOLUME: u32 = 0xc008690c;
const MI_AO_SETMUTE: u32 = 0x4008690d;

pub unsafe fn get_volume_raw(fd: i32) -> io::Result<i32> {
    let mut ioctl_value = [0_i32, 0];
    let ioctl_params = [8, ioctl_value.as_mut_ptr() as usize];
    unsafe { nix::libc::ioctl(fd, MI_AO_GETVOLUME, ioctl_params.clone()) };

    Ok(ioctl_value[1])
}

pub unsafe fn set_volume_raw(fd: i32, mut value: i32) -> io::Result<()> {
    let prev_value = unsafe { get_volume_raw(fd)? };

    value += MIN_RAW_VALUE;

    // Clamp value to the raw ones
    if value > MAX_RAW_VALUE {
        value = MAX_RAW_VALUE;
    } else if value < MIN_RAW_VALUE {
        value = MIN_RAW_VALUE;
    }

    if value == prev_value {
        return Ok(());
    }

    let mut ioctl_value = [0_i32, 0];
    let ioctl_params = [8, ioctl_value.as_mut_ptr() as usize];
    ioctl_value[1] = value;
    unsafe { nix::libc::ioctl(fd, MI_AO_SETVOLUME, ioctl_params.clone()) };

    if prev_value <= MIN_RAW_VALUE && value > MIN_RAW_VALUE {
        ioctl_value[1] = 0;
        unsafe { nix::libc::ioctl(fd, MI_AO_SETMUTE, ioctl_params.clone()) };
    } else if prev_value > MIN_RAW_VALUE && value <= MIN_RAW_VALUE {
        ioctl_value[1] = 1;
        unsafe { nix::libc::ioctl(fd, MI_AO_SETMUTE, ioctl_params.clone()) };
    }

    Ok(())
}

/// Returns volume between 0 and 20
pub async fn get_volume() -> io::Result<i32> {
    let ao = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/mi_ao")
        .await?;
    let fd = ao.as_raw_fd();

    Ok(unsafe { get_volume_raw(fd)? })
}

/// volume should be between 0 and 20
pub async fn set_volume(mut volume: i32) -> io::Result<()> {
    let mut raw = 0;

    if volume > MAX_VOLUME {
        volume = MAX_VOLUME;
    } else if volume < MIN_VOLUME {
        volume = MIN_VOLUME;
    }

    if volume != 0 {
        raw = (48.0 * f64::log10(1.0 + volume as f64)).round() as i32;
    }

    let ao = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/mi_ao")
        .await?;
    let fd = ao.as_raw_fd();

    unsafe { set_volume_raw(fd, raw) }
}
