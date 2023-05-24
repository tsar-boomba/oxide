pub mod error;

use std::{io, path::Path};

pub use error::Error;
use miyoo_mini_hal::charger;
use tokio::task::spawn_blocking;

pub async fn launch() -> Result<(), Error> {
    // Copy .tmp_update from app dir to root of sd card
    match copy_recursively("./.tmp_update", "/mnt/SDCARD/.tmp_update").await {
        Ok(_) => {}
        Err(err) => match err.kind() {
            io::ErrorKind::AlreadyExists => {}
            _ => return Err(err.into()),
        },
    };

    charger::init_charger_detection().await.unwrap();

    Ok(())
}

/// Copy files from source to destination recursively.
fn copy_recursively_sync(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> io::Result<()> {
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively_sync(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Run copy_recursively_sync on the thread pool
async fn copy_recursively(
    source: impl AsRef<Path> + Send + 'static,
    destination: impl AsRef<Path> + Send + 'static,
) -> io::Result<()> {
    match spawn_blocking::<_, io::Result<()>>(|| copy_recursively_sync(source, destination)).await {
        Ok(res) => res,
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "background task failed",
        )),
    }
}
