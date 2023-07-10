use std::{io, sync::atomic::AtomicBool};
use sysinfo::{Pid, ProcessExt, ProcessStatus, Signal, SystemExt};

use miyoo_mini_hal::screen;

use crate::{asyncify, SYSTEM};

static SUSPENDED: AtomicBool = AtomicBool::new(false);

/// Preforms all needed operations to sleep the device (screen off etc.)
pub async fn sleep() -> io::Result<()> {
    SUSPENDED.store(true, std::sync::atomic::Ordering::Relaxed);
    screen::turn_off_screen().await?;
    stop_all_processes().await?;
    Ok(())
}

/// Preforms all needed operations to wake the device (screen on etc.)
pub async fn wake() -> io::Result<()> {
    SUSPENDED.store(false, std::sync::atomic::Ordering::Relaxed);
    cont_all_processes().await?;
    screen::turn_on_screen().await?;
    Ok(())
}

pub fn sleeping() -> bool {
    SUSPENDED.load(std::sync::atomic::Ordering::Relaxed)
}

static EXCLUDE_PROCS: [&'static str; 5] = ["sh", "MainUI", "updater", "launch.sh", "tee"];

async fn stop_all_processes() -> io::Result<()> {
    asyncify(move || {
        let mut system = SYSTEM.lock();
        system.refresh_processes();

        for (_, proc) in system.processes() {
            let status = proc.status();
            let name = proc.name();

            match status {
                ProcessStatus::Run | ProcessStatus::Sleep | ProcessStatus::Dead
                    if EXCLUDE_PROCS
                        .iter()
                        .find(|exclude| name == **exclude)
                        .is_none()
                        // Pid 2 or less will not be stopped
                        && <Pid as Into<usize>>::into(proc.pid()) > 2
                        // Pid parent of 2 or less will not be stopped or 
                        && proc
                            .parent()
                            .map_or(true, |parent| <Pid as Into<usize>>::into(parent) > 2) =>
                {
                    let res = proc.kill_with(Signal::Stop);
                    match res {
                        Some(_) => {}
                        None => tracing::error!("Signal Stop doesnt exist?"),
                    }
                }
                _ => {}
            };
        }

        Ok(())
    })
    .await
}

async fn cont_all_processes() -> io::Result<()> {
    asyncify(move || {
        let mut system = SYSTEM.lock();
        system.refresh_processes();

        for (_, proc) in system.processes() {
            if proc.status() == ProcessStatus::Stop {
                let res = proc.kill_with(Signal::Continue);
                match res {
                    Some(_) => {}
                    None => tracing::error!("Signal Continue doesnt exist?"),
                };
            }
        }

        Ok(())
    })
    .await
}
