use std::time::{Duration, Instant};

use input::{input_task, Button, ButtonEvent};
use miyoo_mini_hal::rumble::rumble_for_sync;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};

use crate::{
    emulator,
    sleep::{sleep, sleeping, wake},
    SystemMessage,
};

const POWER_OFF_TIME: u64 = 500;
const MAIN_MENU_TIME: u64 = 500;

struct ButtonState {
    pressed_at: Option<Instant>,
    cancel_rumble: Option<oneshot::Sender<()>>,
}

pub async fn input(button_sender: mpsc::Sender<SystemMessage>) {
    let mut power_state = ButtonState {
        pressed_at: None,
        cancel_rumble: None,
    };

    let mut menu_state = ButtonState {
        pressed_at: None,
        cancel_rumble: None,
    };

    let (mut button_recv, task) = input_task().await.unwrap();

    tokio::spawn(task);

    // Wait a bit before starting to take input
    tokio::time::sleep(Duration::from_millis(250)).await;
    while let Some(event) = button_recv.recv().await {
        match event.button() {
            // Handle power presses here, they should not be sent to ui
            Button::Power => handle_power(event, &button_sender, &mut power_state).await,
            // Handle menu presses here, only if not sleeping
            Button::Menu if !sleeping() => {
                handle_menu(event, &button_sender, &mut menu_state).await
            }
            // Only send button events to ui if not asleep and not playing game
            _ if !sleeping() && !emulator::playing() => button_sender
                .send(SystemMessage::ButtonEvent(event))
                .await
                .unwrap(),
            _ => {}
        }
    }

    panic!("Main input task ended!");
}

async fn handle_menu(
    event: ButtonEvent,
    button_sender: &mpsc::Sender<SystemMessage>,
    state: &mut ButtonState,
) {
    if event.pressed() {
        state.pressed_at = Some(Instant::now());
        let (cancel, canceled) = oneshot::channel::<()>();
        state.cancel_rumble = Some(cancel);
        tokio::spawn(async move {
            select!(
                _ = tokio::time::sleep(Duration::from_millis(MAIN_MENU_TIME)) => {
                    if let Err(err) = rumble_for_sync(Duration::from_millis(50)).await {
                        tracing::error!("Failed to menu rumble: {err:?}");
                    }
                },
                _ = canceled => {
                    // Rumble was canceled, do nuthin
                }
            )
        });
        return;
    }

    if event.released() {
        if let Some(menu_pressed_at) = state.pressed_at.as_ref() {
            let held_for = Instant::now().duration_since(*menu_pressed_at);

            // If playing, make sure its saved
            if emulator::playing() {
                match emulator::stop_playing().await {
                    Ok(_) => {}
                    Err(err) => tracing::error!("Failed to save: {err:?}"),
                };
            }

            if held_for >= Duration::from_millis(MAIN_MENU_TIME) {
                // Held for long enough, main menu
                button_sender.send(SystemMessage::MainMenu).await.unwrap();
            } else {
                // Shorter,
                button_sender.send(SystemMessage::Switcher).await.unwrap();
            }

            // Reset power pressed at
            state.pressed_at = None;
            state
                .cancel_rumble
                .take()
                .map(|sender| sender.send(()).ok());
        }
    }
}

async fn handle_power(
    event: ButtonEvent,
    button_sender: &mpsc::Sender<SystemMessage>,
    state: &mut ButtonState,
) {
    if event.pressed() {
        state.pressed_at = Some(Instant::now());
        let (cancel, canceled) = oneshot::channel::<()>();
        state.cancel_rumble = Some(cancel);
        tokio::spawn(async move {
            select!(
                _ = tokio::time::sleep(Duration::from_millis(POWER_OFF_TIME)) => {
                    // 3 secs passed since power press, rumble it
                    if let Err(err) = rumble_for_sync(Duration::from_millis(50)).await {
                        tracing::error!("Failed to power rumble: {err:?}");
                    }
                },
                _ = canceled => {
                    // Rumble was canceled, do nuthin
                }
            )
        });
        return;
    }

    if event.released() {
        if let Some(power_pressed_at) = state.pressed_at.as_ref() {
            let held_for = Instant::now().duration_since(*power_pressed_at);

            if held_for >= Duration::from_millis(POWER_OFF_TIME) {
                // Held for long enough, shutdown

                tracing::debug!("Shutting down...");
                button_sender.send(SystemMessage::Shutdown).await.unwrap();

                // If playing. make sure emulator saved
                if emulator::playing() {
                    match emulator::stop_playing().await {
                        Ok(_) => {}
                        Err(err) => tracing::error!("Failed to save: {err:?}"),
                    };
                } else {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }

                nix::unistd::sync();
                tokio::time::sleep(Duration::from_millis(300)).await;
                std::process::Command::new("poweroff").output().unwrap();
                loop {}
            } else {
                // Shorter, sleep/wake
                if sleeping() {
                    tracing::debug!("Waking...");
                    wake().await.ok();
                    button_sender.send(SystemMessage::Wake).await.unwrap();
                } else {
                    println!("sleeping");
                    tracing::debug!("Sleeping...");
                    sleep().await.ok();
                    button_sender.send(SystemMessage::Sleep).await.unwrap();
                }
            }

            // Reset power pressed at
            state.pressed_at = None;
            state
                .cancel_rumble
                .take()
                .map(|sender| sender.send(()).ok());
        }
    }
}
