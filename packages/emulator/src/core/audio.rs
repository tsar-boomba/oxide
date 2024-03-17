use std::sync::atomic::AtomicBool;

use once_cell::sync::OnceCell;

static AUDIO_SENDER: OnceCell<crossbeam::channel::Sender<Vec<i16>>> = OnceCell::new();
static mut AUDIO: Option<Vec<i16>> = None;
static PROCESSING: AtomicBool = AtomicBool::new(false);

pub(super) unsafe extern "C" fn handle_audio_sample(data: *const i16, frames: usize) -> usize {
    let data = std::slice::from_raw_parts(data, frames * 2).to_vec();
    AUDIO_SENDER.get().unwrap().try_send(data).ok();
    tracing::info!("processed audio for {frames} frames!!");
    frames
}

pub fn init() {
    let (sender, receiver) = crossbeam::channel::bounded::<Vec<i16>>(1);
    let mut dsp = oss::Device::new(
        "/dev/dsp",
        2,
        super::av_info().timing.sample_rate.round() as i32,
    )
    .unwrap();
    // Play will block until it is done
    dsp.play_until_empty().unwrap();

    std::thread::spawn(move || loop {
        let data = receiver.recv().unwrap();
        dsp.play(&data).unwrap();
    });

    AUDIO_SENDER.set(sender).unwrap();
}
