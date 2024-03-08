use std::sync::Arc;

use iced::{
    border::Radius,
    color, theme,
    widget::{column, container, row, scrollable, text, Text},
    Background, Border, Command, Element, Length,
};
use input::Button;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use shared_ui::{scrollable_list, ListItem, ScrollableList};
use system::{
    emulator::play,
    games::{Console, Game},
    SystemMessage,
};

use crate::{app::App, layout::layout, Message};

use super::Screen;

static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug)]
pub struct State {
    consoles: Vec<Console>,
    selected_console: Option<Console>,
    game_list: ScrollableList<App>,
    /// idx of selected button
    selected: usize,
    page: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            consoles: Console::iter().collect(),
            game_list: ScrollableList::new(vec![]),
            selected_console: None,
            selected: 0,
            page: 0,
        }
    }
}

impl State {
    pub fn update(app: &mut App, message: Message) -> Command<Message> {
        let mut state = STATE.lock();
        match &message {
            Message::System(SystemMessage::ButtonEvent(ev)) => {
                if state.selected_console.is_some() {
                    // Console Selected
                    match ev.button() {
                        Button::B if ev.pressed() => {
                            // Go back to console menu
                            state.selected_console = None;
                            Command::none()
                        }
                        Button::Up if ev.pressed() => {
                            state
                                .game_list
                                .update(app, message, scrollable_list::Message::Up)
                        }
                        Button::Down if ev.pressed() => {
                            state
                                .game_list
                                .update(app, message, scrollable_list::Message::Down)
                        }
                        _ => state
                            .game_list
                            .update(app, message, scrollable_list::Message::Other),
                    }
                } else {
                    // No console selected
                    match ev.button() {
                        Button::A if ev.pressed() => {
                            // Select Console
                            let selected = state.consoles[state.selected];
                            state.selected_console = Some(selected);
                            let console_games: Arc<[Game]> =
                                app.games.get(selected).cloned().unwrap();

                            // Construct game list for selected console, giving each callback its own copy of games
                            state.game_list = ScrollableList::new(
                                (0..console_games.len())
                                    .into_iter()
                                    .map(move |game_idx| {
                                        // Make one clone for each callback
                                        let children_games = console_games.clone();
                                        let action_games = console_games.clone();

                                        ListItem::new(
                                            move |app: &'_ App| {
                                                let game = &children_games[game_idx];
                                                text(game.full_name()).size(20).into()
                                            },
                                            move |app: &'_ mut App, message: Message| {
                                                let game = &action_games[game_idx];

                                                match message {
                                                    Message::System(
                                                        SystemMessage::ButtonEvent(ev),
                                                    ) => match ev.button() {
                                                        Button::A if ev.pressed() => {
                                                            play(game);
                                                            app.screen = Screen::Playing(Some(
                                                                Box::new(Screen::Games),
                                                            ));

                                                            Command::none()
                                                        }
                                                        _ => Command::none(),
                                                    },
                                                    _ => Command::none(),
                                                }
                                            },
                                        )
                                    })
                                    .collect(),
                            );
                            Command::none()
                        }
                        Button::B if ev.pressed() => {
                            app.screen = Screen::Main;
                            Command::none()
                        }
                        Button::Right if ev.pressed() => {
                            // Move to right if possible
                            if state.selected < state.consoles.len() - 1 {
                                state.selected += 1;
                            };
                            Command::none()
                        }
                        Button::Left if ev.pressed() => {
                            // Move to left if possible
                            if state.selected > 0 {
                                state.selected -= 1;
                            };
                            Command::none()
                        }
                        _ => Command::none(),
                    }
                }
            }
            _ => Command::none(),
        }
    }

    pub fn view(app: &App) -> Element<Message> {
        let state: MutexGuard<'static, State> = STATE.lock();
        layout(
            app,
            if state.selected_console.is_some() {
                state.game_list.view(app)
            } else {
                row(state
                    .consoles
                    .iter()
                    // TODO: remove cloning here somehow????
                    .copied()
                    .enumerate()
                    .map(|(i, console)| console_view(console, i == state.selected)))
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
            },
        )
    }
}

fn console_view(console: Console, selected: bool) -> Element<'static, Message> {
    container(
        column![text("icon here").size(32), text(console.name())]
            .align_items(iced::Alignment::Center)
            .spacing(8)
            .width(Length::Fill)
            .padding(8),
    )
    .style(move |theme: &'_ iced::Theme| -> container::Appearance {
        container::Appearance {
            border: Border {
                radius: Radius::from(8.),
                width: 2.,
                color: color!(255, 0, 0),
            },
            background: if selected {
                Some(Background::Color(color!(0, 0, 255)))
            } else {
                None
            },
            ..Default::default()
        }
    })
    .into()
}
