use std::io;

use once_cell::sync::OnceCell;
use serde::Deserialize;

use crate::{
    gpio::{Input, Pin},
    model::{model, Model},
};

static PIN: OnceCell<Pin<Input>> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct BatteryInfo {
    /// Battery percentage
    pub battery: u8,
    pub voltage: u32,
    /// 0 or 1
    pub charging: u8,
}

/// This initializes charger detection using gpio on mini only
pub async fn init_charger_detection() -> io::Result<()> {
    match model().await? {
        // Initialize pin 59 as input
        Model::Mini => pin().await.map(|_| ()),
        Model::MiniPlus => Ok(()),
    }
}

pub async fn battery_info() -> io::Result<BatteryInfo> {
    let model = model().await?;
    match model {
        Model::Mini => todo!(),
        Model::MiniPlus => {
            // TODO: find out why tokio::process:Command breaks everything
            let mut output = std::process::Command::new("/customer/app/axp_test").output()?;
            let info = serde_json::from_slice(&output.stdout).unwrap();
            Ok(info)
        }
    }
}

async fn pin() -> io::Result<&'static Pin<Input>> {
    match PIN.get() {
        Some(pin) => Ok(pin),
        None => {
            let pin = Pin::<Input>::new_input(59).await?;
            PIN.set(pin).ok();
            Ok(PIN.get().unwrap())
        }
    }
}
