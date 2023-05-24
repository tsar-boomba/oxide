use std::io;

use once_cell::sync::OnceCell;
use tokio_gpiod::{Chip, DirectionType, LineId, Lines, Options, Output, Input};

pub const GPIO_CHIP_PATH: &str = "/sys/devices/gpiochip0";

static CHIP: OnceCell<Chip> = OnceCell::new();

pub(crate) async fn request_lines<Direction: DirectionType>(
    options: Options<Direction, impl AsRef<[LineId]>, impl AsRef<str>>,
) -> io::Result<Lines<Direction>> {
    chip().await?.request_lines(options).await
}

pub(crate) async fn output_lines(
    options: Options<Output, impl AsRef<[LineId]>, impl AsRef<str>>,
) -> io::Result<Lines<Output>> {
    request_lines(options).await
}

pub(crate) async fn input_lines(
    options: Options<Input, impl AsRef<[LineId]>, impl AsRef<str>>,
) -> io::Result<Lines<Input>> {
    request_lines(options).await
}

/// Asynchronously gets or initializes the gpio chip
async fn chip() -> io::Result<&'static Chip> {
    match CHIP.get() {
        Some(chip) => Ok(chip),
        None => {
            let chip = Chip::new(GPIO_CHIP_PATH).await?;
            CHIP.set(chip).ok();
            
            Ok(CHIP.get().unwrap())
        }
    }
}
