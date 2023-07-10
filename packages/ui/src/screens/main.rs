use std::sync::Arc;

use iced::{
    color, theme,
    widget::{column, container, row, runtime::core::BorderRadius, text, Text},
    Command, Element, Length,
};
use input::Button;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use system::SystemMessage;

use crate::{app::App, layout::layout, Message};

use super::Screen;

static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug, Clone)]
struct MainScreenButton {
    text: &'static str,
    icon: String,
    screen: Screen,
}

#[derive(Debug)]
pub struct State {
    buttons: Vec<MainScreenButton>,
    /// idx of selected button
    selected: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            buttons: vec![
                MainScreenButton {
                    icon: "".into(),
                    text: "favorites",
                    screen: Screen::Favorites,
                },
                MainScreenButton {
                    icon: "".into(),
                    text: "games",
                    screen: Screen::Games,
                },
                MainScreenButton {
                    icon: "".into(),
                    text: "settings",
                    screen: Screen::Settings,
                },
            ],
            selected: 0,
        }
    }
}

impl State {
    pub fn update(app: &mut App, message: Message) -> Command<Message> {
        let mut state = STATE.lock();
        match message {
            Message::System(SystemMessage::ButtonEvent(ev)) => {
                match ev.button() {
                    Button::A if ev.pressed() => {
                        // Change screen to selected button
                        app.screen = state.buttons[state.selected].screen.clone();
                    }
                    Button::Right if ev.pressed() => {
                        // Move to right if possible
                        if state.selected < state.buttons.len() - 1 {
                            state.selected += 1;
                        }
                    }
                    Button::Left if ev.pressed() => {
                        // Move to left if possible
                        if state.selected > 0 {
                            state.selected -= 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Command::none()
    }

    pub fn view(app: &App) -> Element<Message> {
        let state: MutexGuard<'static, State> = STATE.lock();
        layout(
            app,
            row(state
                .buttons
                .iter()
                // TODO: remove cloning here somehow????
                .cloned()
                .enumerate()
                .map(|(i, button)| button.view(i == state.selected))
                .collect())
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(16)
            .into(),
        )
    }
}

impl MainScreenButton {
    pub fn view(self, selected: bool) -> Element<'static, Message> {
        container(
            column![
                text("icon here").size(32),
                text(format!(
                    "{}{}",
                    self.text,
                    // TODO: switch this for a border or some other way to indicate selection
                    if selected { " selected" } else { "" }
                ))
            ]
            .align_items(iced::Alignment::Center)
            .spacing(8)
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(8),
        )
        .style(theme::Container::Box)
        .into()
    }
}
