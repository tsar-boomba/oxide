use iced::{
    color,
    widget::{container, text},
    window, Command, Element, Length,
};
use system::{Init, SystemMessage};

use crate::{app::App, Message};

pub mod favorites;
pub mod games;
pub mod main;
mod playing;
pub mod settings;
pub mod switcher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Startup,
    Shutdown,
    /// Displays nothing, holds previous state, use option to remove need for cloning on wake
    Sleep(Option<Box<Screen>>),
    Playing(Option<Box<Screen>>),
    Main,
    Favorites,
    Games,
    Settings,
    Switcher,
}

impl Screen {
    pub fn update(app: &mut App, message: Message) -> Command<Message> {
        match app.screen {
            Self::Startup => Command::none(),
            Self::Sleep(ref mut prev) => {
                match message {
                    // Unwrap should be safe as prev should never be none
                    Message::System(SystemMessage::Wake) => app.screen = *prev.take().unwrap(),
                    _ => {}
                };
                Command::none()
            }
            Self::Playing(ref mut prev) => {
                playing::State::set_prev(prev);
                playing::State::update(app, message)
            }
            Self::Main => main::State::update(app, message),
            Self::Favorites => favorites::State::update(app, message),
            Self::Games => games::State::update(app, message),
            Self::Settings => settings::State::update(app, message),
            Self::Switcher => switcher::State::update(app, message),
            Self::Shutdown => Command::none(),
        }
    }

    pub fn view(app: &App) -> Element<Message> {
        match app.screen {
            Self::Startup => container::Container::new(text("Starting..."))
                .center_x()
                .center_y()
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            Self::Sleep(_) => container::Container::new(text("sleeping... zzzzz"))
                .center_x()
                .center_y()
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            Self::Playing(_) => playing::State::view(app),
            Self::Main => main::State::view(app),
            Self::Favorites => favorites::State::view(app),
            Self::Games => games::State::view(app),
            Self::Settings => settings::State::view(app),
            Self::Switcher => switcher::State::view(app),
            Self::Shutdown => container::Container::new(text("powering off..."))
                .center_x()
                .center_y()
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::Startup
    }
}
