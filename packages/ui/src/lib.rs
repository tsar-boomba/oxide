mod app;
pub mod message;
mod screens;

use app::App;
use iced::{Application, Settings, Font};
pub use message::Message;

/// Starts UI in a dedicated thread
pub fn start() {
    App::run(Settings {
        window: iced::window::Settings {
            visible: true,
            resizable: false,
            decorations: false,
            always_on_top: true,
            size: (640, 480),
            ..Default::default()
        },
        flags: (),
        exit_on_close_request: false,
        antialiasing: false,
        default_font: Font::DEFAULT,
        default_text_size: 20.0,
        id: None,
    })
    .unwrap()
}
