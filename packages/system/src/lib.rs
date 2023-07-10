mod battery;
pub mod emulator;
pub mod games;
mod input_task;
pub mod settings;
pub mod sleep;

use std::future::Future;

use ::input::ButtonEvent;
use battery::battery;
use futures_util::future::join;
use games::GameCache;
use input_task::input;
use launch::launch;
use miyoo_mini_hal::model::{model, Model};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use sysinfo::{System, SystemExt};
use tokio::sync::mpsc;

pub use settings::Settings;

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemMessage {
    ButtonEvent(ButtonEvent),
    BatteryPercentage(u8),
    Error(String),
    Sleep,
    Wake,
    Shutdown,
    MainMenu,
    Switcher,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Init {
    pub model: Model,
    pub settings: Settings,
    pub games: GameCache,
}

/// Starts a background task that handles system stuff like battery percentage, input, etc
pub fn task() -> (
    mpsc::Receiver<SystemMessage>,
    impl Future<Output = Init> + Send + 'static,
) {
    let (event_sender, event_receiver) = mpsc::channel(64);

    (event_receiver, async move {
        let (settings, games) = join(
            async move {
                launch().await.unwrap();

                tokio::spawn(input(event_sender.clone()));

                tokio::spawn(battery(event_sender.clone()));

                let (settings, settings_recv) = Settings::init().await.unwrap();
                tokio::spawn(settings::task(settings_recv));

                tokio::spawn(emulator::task(event_sender.clone()));

                settings
            },
            games::init(),
        )
        .await;

        Init {
            model: model().await.unwrap(),
            settings,
            games: games.unwrap(),
        }
    })
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
