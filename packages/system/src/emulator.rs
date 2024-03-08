use std::sync::atomic::{AtomicBool, Ordering};

use ipc::functions::{SaveState, SaveStateArgs};
use once_cell::sync::OnceCell;
use tokio::sync::mpsc;

use crate::{games::Game, SystemMessage};

static SENDER: OnceCell<mpsc::Sender<Option<tokio::process::Child>>> = OnceCell::new();
static PLAYING: AtomicBool = AtomicBool::new(false);

pub fn playing() -> bool {
    PLAYING.load(Ordering::Relaxed)
}

/// Called from ui to start the emulator
pub fn play(game: &Game) {
    tracing::debug!("Playing game: {}", game.as_path().display());

    let proc = tokio::process::Command::new("emulator")
        .args([
            format!("/mnt/SDCARD/Cores/{}_libretro.so", game.core()),
            format!("{}", game.as_path().display()),
            "--load-auto".into(),
        ])
        .spawn()
        .unwrap();

    SENDER.get().unwrap().try_send(Some(proc)).unwrap();

    PLAYING.store(true, Ordering::Relaxed);
}

/// Stops emulator proc along with creating a save state
pub async fn stop_playing() -> Result<(), String> {
    if !PLAYING.load(Ordering::Relaxed) {
        tracing::warn!("Tried to kill emulator while not playing.");
        return Ok(());
    }

    // Firstly, tell emulator to save into the auto slot
    let _ = match ipc::client::call::<SaveState>(SaveStateArgs { slot: None }).await {
        Ok(res) => {
            if res.status().is_server_error() {
                // Failed again
                tracing::error!("Emulator error while saving.");
                Err("Error while saving state".to_string())
            } else {
                Ok(())
            }
        }
        Err(err) => {
            tracing::error!("Hyper error while saving: {err:?}");
            Err("Error while saving state".to_string())
        }
    }?;

    // Now, tell task to kill the emulator process
    SENDER.get().unwrap().try_send(None).unwrap();

    // Set flag
    PLAYING.store(false, Ordering::Relaxed);

    Ok(())
}

pub(crate) async fn task(event_sender: mpsc::Sender<SystemMessage>) {
    let (proc_sender, mut proc_recv) = mpsc::channel(1);
    let mut proc_id: Option<u32> = None;

    SENDER.set(proc_sender).unwrap();

    while let Some(proc) = proc_recv.recv().await {
        match proc {
            Some(mut proc) => {
                tracing::debug!("Starting an emulator proc");
                let event_sender = event_sender.clone();
                proc_id = proc.id();

                // Monitor the status of the emulator process
                tokio::spawn(async move {
                    tracing::debug!("Started emu watch task.");
                    let status = proc.wait().await;
                    tracing::debug!("Emulator proc ended.");
                    PLAYING.store(false, Ordering::Relaxed);
                    let status = status.unwrap();
                    if let Some(code) = status.code() {
                        // It was only a crash if there is an exit code
                        // Otherwise it was killed by a signal which is intended
                        tracing::debug!("Emulator proc crashed.");
                        event_sender
                            .send(SystemMessage::Error(format!(
                                "Emulator crashed with status: {}",
                                code
                            )))
                            .await
                            .unwrap();
                    }
                });
            }
            None => {
                // Terminate emulator proc
                if let Some(pid) = proc_id.take() {
                    // SAFETY: Idk, it probably is
                    unsafe {
                        nix::libc::kill(pid as i32, nix::libc::SIGKILL);
                    }
                } else {
                    tracing::error!("Tried to kill emulator while no process running!");
                }
            }
        }
    }

    panic!("Emulator task ended!!");
}
