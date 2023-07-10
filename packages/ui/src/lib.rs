mod app;
mod layout;
pub mod message;
mod screens;

use app::App;
use iced::{window::Position, Application, Font, Settings};
pub use message::Message;

/// Starts UI in a dedicated thread
pub fn start() {
    App::run(Settings {
        window: iced::window::Settings {
            visible: true,
            resizable: false,
            decorations: false,
            position: Position::Specific(0, 0),
            size: (640, 480),
            ..Default::default()
        },
        flags: (),
        exit_on_close_request: false,
        antialiasing: false,
        default_font: Font::DEFAULT,
        default_text_size: 20.0,
        id: Some("oxide-os".into()),
    })
    .unwrap()
}
