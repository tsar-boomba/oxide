use std::sync::Arc;

use iced::{
    color, theme,
    widget::{
        column, container, row,
        text, Text,
    },
    window, Command, Element, Length,
};
use input::Button;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use system::SystemMessage;

use crate::{app::App, layout::layout, Message};

use super::Screen;

static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug, Default)]
pub struct State {
    error: Option<String>,
    prev: Option<Screen>,
}

impl State {
    /// Must be called before update
    pub fn set_prev(prev: &mut Option<Box<Screen>>) {
        if let Some(prev) = prev.take() {
            STATE.lock().prev = Some(*prev);
        }
    }

    /// MUST CALL set_prev BEFORE THIS
    pub fn update(app: &mut App, message: Message) -> Command<Message> {
        let mut state = STATE.lock();
        match message {
            Message::System(SystemMessage::ButtonEvent(ev)) => match ev.button() {
                Button::B if ev.pressed() => {
                    app.screen = state.prev.take().unwrap();
                    Command::none()
                }
                _ => Command::none(),
            },
            Message::System(SystemMessage::Error(err)) => {
                state.error = Some(err);
                // Unminimize because error was likely emulator crash
                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(app: &App) -> Element<Message> {
        let state: MutexGuard<'static, State> = STATE.lock();
        layout(
            app,
            column![
                text(if state.error.is_some() {
                    "An error ocurred!"
                } else {
                    ""
                })
                .size(24)
                .style(color!(255, 0, 0)),
                text(state.error.as_deref().unwrap_or_default()),
                text("Press `B` to go back.")
            ]
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(16)
            .into(),
        )
    }
}
