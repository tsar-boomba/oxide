use std::io;

use i2cdev::{core::*, linux::LinuxI2CDevice};
use once_cell::sync::OnceCell;

/// Store the model of the device this is running on and reuse
static DEVICE_MODEL: OnceCell<Model> = OnceCell::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    Mini,
    MiniPlus,
}

impl Model {
    pub fn width(&self) -> u32 {
        match self {
            Self::Mini => 640,
            Self::MiniPlus => 640,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::Mini => 480,
            Self::MiniPlus => 480,
        }
    }
}

pub async fn model() -> io::Result<Model> {
    match DEVICE_MODEL.get() {
        Some(model) => Ok(*model),
        None => {
            let model = match is_plus().await? {
                true => Model::MiniPlus,
                false => Model::Mini,
            };
            DEVICE_MODEL.set(model).ok();
            Ok(model)
        }
    }
}

/// Do some i2c stuff which will tell us if we are running on a plus
async fn is_plus() -> io::Result<bool> {
    match tokio::task::spawn_blocking(|| {
        let mut device = LinuxI2CDevice::new("/dev/i2c-1", 0x34)?;
        Ok(device.smbus_read_byte_data(0).is_ok())
    })
    .await
    {
        Ok(res) => res,
        Err(err) => {
            tracing::error!("{err}");
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Blocking task for i2c panicked.",
            ))
        }
    }
}
