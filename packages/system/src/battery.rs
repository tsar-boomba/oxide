use std::time::Duration;

use miyoo_mini_hal::charger::battery_info;
use tokio::sync::mpsc;

use crate::{sleep::sleeping, SystemMessage};

/// Task which gets battery info and sends to ui every 10 seconds
pub async fn battery(battery_sender: mpsc::Sender<SystemMessage>) {
    loop {
        let info = battery_info().await.unwrap();

        if !sleeping() {
            // Only update the ui if not sleeping
            battery_sender
                .send(SystemMessage::BatteryPercentage(info.battery))
                .await
                .unwrap();
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
