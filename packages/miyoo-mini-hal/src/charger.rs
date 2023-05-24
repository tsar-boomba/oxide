use std::io;

use tokio_gpiod::Options;

use crate::{
    gpio::input_lines,
    model::{model, Model},
};

/// This initializes charger detection using gpio on mini only
pub async fn init_charger_detection() -> io::Result<()> {
    match model().await? {
		// Initialize pin 59 as input
        Model::Mini => input_lines(Options::input([59])).await.map(|_| {}),
        Model::MiniPlus => Ok(()),
    }
}
