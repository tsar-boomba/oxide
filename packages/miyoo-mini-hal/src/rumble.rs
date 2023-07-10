use std::{io, time::Duration};

use once_cell::sync::OnceCell;

use crate::gpio::{Output, Pin};

static PIN: OnceCell<Pin<Output>> = OnceCell::new();

/// Must be awaited for rumble to take place
pub async fn rumble_for_sync(duration: Duration) -> io::Result<()> {
    let pin = pin().await?;
    pin.set_value(false).await?;
    tokio::time::sleep(duration).await;
    pin.set_value(true).await?;
    Ok(())
}

/// Rumbles for duration in a separate task
pub fn rumble_for_async(duration: Duration) {
    tokio::spawn(async move {
        let res = rumble_for_sync(duration).await;
        if let Err(err) = res {
            tracing::error!("Failed to rumble async: {err:?}");
        }
    });
}

async fn pin() -> io::Result<&'static Pin<Output>> {
    match PIN.get() {
        Some(pin) => Ok(pin),
        None => {
            let pin = Pin::<Output>::new_output(48).await?;
            PIN.set(pin).ok();
            Ok(PIN.get().unwrap())
        }
    }
}
