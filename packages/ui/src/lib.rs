mod app;
mod layout;
pub mod message;
mod screens;

use app::App;
use iced::{window::Position, Application, Font, Pixels, Point, Settings, Size};
pub use message::Message;

/// Starts UI in a dedicated thread
pub fn start() {
    App::run(Settings {
        window: iced::window::Settings {
            visible: true,
            resizable: false,
            decorations: false,
            position: Position::Specific(Point::new(0., 0.)),
            size: Size::new(640., 480.),
            ..Default::default()
        },
        flags: (),
        antialiasing: false,
        default_font: Font::DEFAULT,
        fonts: vec![],
        default_text_size: Pixels(20.),
        id: Some("oxide-os".into()),
    })
    .unwrap()
}
