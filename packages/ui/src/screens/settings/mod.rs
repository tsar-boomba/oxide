use std::{fmt::Debug, sync::Arc};

use iced::{
    color, theme,
    widget::{column, container, row, runtime::core::BorderRadius, text, Text},
    Command, Element, Length,
};
use input::Button;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use shared_ui::{scrollable_list, ListItem, ScrollableList};
use system::SystemMessage;

use crate::{app::App, layout::layout, Message};

use super::Screen;

static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug)]
pub struct State {
    list: ScrollableList<App>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            list: ScrollableList::new(vec![ListItem::new(
                |app: &'_ App| {
                    text(format!("Brightness     {}/10", app.settings.brightness))
                        .size(24)
                        .into()
                },
                |app, message| match message {
                    Message::System(SystemMessage::ButtonEvent(ev)) => match ev.button() {
                        Button::Left if ev.pressed() => {
                            if app.settings.brightness > 0 {
                                app.settings.update(|settings| {
                                    settings.brightness -= 1;
                                });
                            }
                            Command::none()
                        }
                        Button::Right if ev.pressed() => {
                            if app.settings.brightness < 10 {
                                app.settings.update(|settings| {
                                    settings.brightness += 1;
                                });
                            }
                            Command::none()
                        }
                        _ => Command::none(),
                    },
                    _ => Command::none(),
                },
            )]),
        }
    }
}

impl State {
    pub fn update(app: &mut App, message: Message) -> Command<Message> {
        let mut state = STATE.lock();
        match &message {
            Message::System(SystemMessage::ButtonEvent(ev)) => match ev.button() {
                Button::B if ev.pressed() => {
                    app.screen = Screen::Main;
                    Command::none()
                }
                Button::Up if ev.pressed() => {
                    state
                        .list
                        .update(app, message, scrollable_list::Message::Up)
                }
                Button::Down if ev.pressed() => {
                    state
                        .list
                        .update(app, message, scrollable_list::Message::Down)
                }
                _ => state
                    .list
                    .update(app, message, scrollable_list::Message::Other),
            },
            _ => Command::none(),
        }
    }

    pub fn view(app: &App) -> Element<Message> {
        let state: MutexGuard<'static, State> = STATE.lock();
        layout(app, state.list.view(app))
    }
}
