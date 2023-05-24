use std::{future::Future, time::Duration};

use launch::launch;
use tokio::sync::mpsc;
use input::{ButtonEvent, start_input_task, Button};

#[derive(Debug, Clone)]
pub enum SystemMessage {
    ButtonEvent(ButtonEvent),
    BatteryPercentage(u16),
    Sleep,
    Wake,
    Shutdown
}

/// Starts a background task that handles system stuff like battery percentage etc
pub fn task() -> (mpsc::Receiver<SystemMessage>, impl Future<Output = ()> + Send + 'static) {
    let (event_sender, event_receiver) = mpsc::channel(64);
    
    (event_receiver, async move {
        launch().await.unwrap();
        let button_sender = event_sender.clone();
        tokio::spawn(async move {
            while let Some(event) = start_input_task().await.unwrap().recv().await {
                tracing::info!("Press: {event:?}");
                button_sender.send(SystemMessage::ButtonEvent(event.clone())).await.unwrap();

                if event.is_released(Button::Power) {
                    tracing::info!("Shutting down.");
                    nix::unistd::sync();
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    std::process::Command::new("poweroff").output().unwrap();
                    loop {}
                }
            }
        });
    })
}