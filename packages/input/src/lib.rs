pub mod button;
pub mod event;

use std::{io, process, time::Instant};

use evdev::Device;
use futures_util::StreamExt;
use tokio::sync::{mpsc, oneshot};

pub use button::Button;
pub use event::{ButtonEvent, EventValue};

#[derive(Debug)]
pub struct ButtonHandler {
    pressed_at: Option<Instant>,
    cancel: Option<oneshot::Sender<()>>,
    handle_held: fn(),
}

impl ButtonHandler {}

pub async fn input_task() -> io::Result<(
    mpsc::Receiver<ButtonEvent>,
    impl std::future::Future<Output = ()>,
)> {
    let mut stream = asyncify(|| {
        let device = Device::open("/dev/input/event0")?;
        device.into_event_stream()
    })
    .await?;
    let (sender, receiver) = mpsc::channel(64);

    let future = async move {
        loop {
            match stream.next().await {
                Some(res) => match res {
                    Ok(event) => {
                        if let Some(event) = ButtonEvent::from_event(event) {
                            sender.send(event).await.ok();
                        }
                    }
                    Err(err) => {
                        eprint!("Input event stream error: {}", err);
                        process::exit(1);
                    }
                },
                None => {
                    eprintln!("Input event stream stopped unexpectedly.");
                    process::exit(1);
                }
            }
        }
    };

    Ok((receiver, future))
}

/// Spawn a task on the blocking thread pool
pub(crate) async fn asyncify<F, T>(f: F) -> std::io::Result<T>
where
    F: FnOnce() -> std::io::Result<T> + Send + 'static,
    T: Send + 'static,
{
    match tokio::task::spawn_blocking(f).await {
        Ok(res) => res,
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "background task failed",
        )),
    }
}
