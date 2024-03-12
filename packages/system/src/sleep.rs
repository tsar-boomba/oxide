use ipc::functions::{StartArgs, StopArgs};
use std::{
    io::{self, Write},
    sync::atomic::AtomicBool,
};
use sysinfo::{Pid, ProcessStatus, Signal};

use miyoo_mini_hal::screen;

use crate::{asyncify, emulator::playing, SYSTEM};

static SUSPENDED: AtomicBool = AtomicBool::new(false);

/// Preforms all needed operations to sleep the device (screen off etc.)
pub async fn sleep() -> io::Result<()> {
    if playing() {
        ipc::client::call::<ipc::functions::Stop>(StopArgs {})
            .await
            .unwrap();
    }

    stop_all_processes().await?;
    screen::turn_off_screen().await?;
    SUSPENDED.store(true, std::sync::atomic::Ordering::Relaxed);
    Ok(())
}

/// Preforms all needed operations to wake the device (screen on etc.)
pub async fn wake() -> io::Result<()> {
    continue_all_processes().await?;

    if playing() {
        ipc::client::call::<ipc::functions::Start>(StartArgs {})
            .await
            .unwrap();
    }

    screen::turn_on_screen().await?;
    SUSPENDED.store(false, std::sync::atomic::Ordering::Relaxed);
    Ok(())
}

pub fn sleeping() -> bool {
    SUSPENDED.load(std::sync::atomic::Ordering::Relaxed)
}

static EXCLUDE_PROCS: &'static [&'static str] = &[
    "sh",
    "emulator",
    "smithay-clipboa",
    "weston-desktop-",
    "tokio-runtime-w",
    "MainUI",
    "updater",
    "launch.sh",
    "tee",
];

async fn stop_all_processes() -> io::Result<()> {
    asyncify(move || {
        let mut system = SYSTEM.lock();
        system.refresh_processes();

        let processes = system.processes();
        println!("proces: {processes:#?}");
        std::io::stdout().lock().flush().ok();
        for (pid, proc) in processes {
            let status = proc.status();
            let name = proc.name();

            match status {
                ProcessStatus::Run | ProcessStatus::Sleep | ProcessStatus::Dead
                    // Stop if not excluded
                    if EXCLUDE_PROCS
                        .iter()
                        .find(|exclude| name == **exclude)
                        .is_none()
                        // Pid 2 or less will not be stopped
                        && <Pid as Into<usize>>::into(*pid) > 2
                        // Pid parent of 2 or less will not be stopped
                        && proc
                            .parent()
                            .map_or(true, |parent| <Pid as Into<usize>>::into(parent) > 2) =>
                {
                    println!("Stopping: {}", name);
                    let res = proc.kill_with(Signal::Stop);
                    match res {
                        Some(_) => {}
                        None => tracing::error!("Signal Stop doesn't exist?"),
                    }
                }
                _ => {}
            };
        }

        Ok(())
    })
    .await
}

async fn continue_all_processes() -> io::Result<()> {
    asyncify(move || {
        let mut system = SYSTEM.lock();
        system.refresh_processes();

        for (_, proc) in system.processes() {
            if proc.status() == ProcessStatus::Stop {
                tracing::debug!("Restarting process: {}", proc.name());
                let res = proc.kill_with(Signal::Continue);
                match res {
                    Some(_) => {}
                    None => tracing::error!("Signal Continue doesn't exist?"),
                };
            }
        }

        Ok(())
    })
    .await
}
