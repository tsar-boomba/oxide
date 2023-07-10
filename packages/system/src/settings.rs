use std::io;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    sync::mpsc,
};

pub(crate) static SETTINGS_SENDER: OnceCell<mpsc::Sender<Settings>> = OnceCell::new();

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Settings {
    pub brightness: u8,
    pub volume: u8,
}

impl Settings {
    pub async fn init() -> io::Result<(Self, mpsc::Receiver<Self>)> {
        let mut settings_string = String::with_capacity(16 * 16);
        let mut file = tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("settings.json")
            .await?;
        let bytes_read = file.read_to_string(&mut settings_string).await?;

        let settings = match serde_json::from_str::<Settings>(&settings_string) {
            Ok(settings) => {
                // Found valid settings file
                tracing::debug!("Found valid settings: {settings:?}");
                settings
            }
            Err(_) if bytes_read == 0 => {
                // No / empty settings file
                tracing::debug!("No / empty settings.");
                let settings = Settings::default();
                file.write_all(&serde_json::to_vec_pretty(&settings).unwrap())
                    .await?;
                settings
            }
            Err(err) => {
                // Invalid settings file
                let settings = Settings::default();
                file.write_all(&serde_json::to_vec_pretty(&settings).unwrap())
                    .await?;
                tracing::error!("Found invalid settings file. Using default. {err:?}");
                settings
            }
        };

        let (send, recv) = mpsc::channel(16);
        SETTINGS_SENDER.set(send).ok();

        Ok((settings, recv))
    }

    /// Use this to update settings as it will inform the respective system task
    pub fn update(&mut self, op: impl FnOnce(&mut Self)) {
        op(self);
        SETTINGS_SENDER
            .get()
            .unwrap()
            .try_send(self.clone())
            .unwrap();
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            brightness: 6,
            volume: 8,
        }
    }
}

/// Handle settings updates from the ui
pub async fn task(mut recv: mpsc::Receiver<Settings>) {
    let mut prev_brightness: Option<u8> = None;
    let mut prev_volume: Option<u8> = None;
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("settings.json")
        .await
        .unwrap();

    loop {
        while let Some(new_settings) = recv.recv().await {
            if prev_brightness != Some(new_settings.brightness) {
                // Brightness update

                prev_brightness = Some(new_settings.brightness);
            }

            if prev_volume != Some(new_settings.volume) {
                // Volume update

                prev_volume = Some(new_settings.volume)
            }

            // Save to file
            file.seek(io::SeekFrom::Start(0)).await.unwrap();
            if let Err(err) = file
                .write_all(&serde_json::to_vec_pretty(&new_settings).unwrap())
                .await
            {
                tracing::error!("Failed to save settings: {err:?}")
            }
        }
    }
}
