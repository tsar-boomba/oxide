use std::io;

use once_cell::sync::OnceCell;
use tokio_sysfs_pwm::{Chip, Pin};

static CHIP: OnceCell<Chip> = OnceCell::new();

pub(crate) async fn chip() -> io::Result<&'static Chip> {
    match CHIP.get() {
        Some(chip) => Ok(chip),
        None => {
            let chip = Chip::new(0).await?;
            CHIP.set(chip).ok();

            Ok(CHIP.get().unwrap())
        }
    }
}

pub(crate) async fn pin(pin_num: u32) -> io::Result<Pin<'static>> {
    chip().await?.pin(pin_num).await
}
