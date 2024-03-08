use once_cell::sync::OnceCell;

static AUDIO_SENDER: OnceCell<crossbeam::channel::Sender<Vec<i16>>> = OnceCell::new();

pub(super) unsafe extern "C" fn handle_audio_sample(data: *const i16, frames: usize) -> usize {
    let data = std::slice::from_raw_parts(data, frames * 2).to_vec();
    AUDIO_SENDER.get().unwrap().send(data).ok();
    frames
}

pub fn init() {
    let (sender, receiver) = crossbeam::channel::bounded(32);
    AUDIO_SENDER.set(sender).unwrap();
}
