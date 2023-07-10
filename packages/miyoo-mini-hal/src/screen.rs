use std::io;

use once_cell::sync::OnceCell;
use tokio_sysfs_pwm::Pin;

use crate::pwm;

static SCREEN_PIN: OnceCell<tokio::sync::Mutex<Pin<'static>>> = OnceCell::new();

/// Sets the pwm stuff to enable the backlight
pub async fn change_brightness() -> io::Result<()> {
    let mut pin = pin().await?;
    pin.set_period_ns(800).await?;
    pin.set_duty_cycle_ns(6).await?;
    pin.enable().await?;
    Ok(())
}

pub async fn turn_off_screen() -> io::Result<()> {
    let mut pin = pin().await?;
    pin.disable().await?;
    Ok(())
}

pub async fn turn_on_screen() -> io::Result<()> {
    let mut pin = pin().await?;
    pin.enable().await?;
    Ok(())
}

async fn pin() -> io::Result<tokio::sync::MutexGuard<'static, Pin<'static>>> {
    match SCREEN_PIN.get() {
        Some(pin) => Ok(pin.lock().await),
        None => {
            SCREEN_PIN
                .set(tokio::sync::Mutex::new(pwm::pin(0).await?))
                .ok();
            Ok(SCREEN_PIN.get().unwrap().lock().await)
        }
    }
}
