use std::cell::RefCell;

use iced::{
    executor, subscription,
    widget::runtime::{command::Action, window},
    window::Mode,
    Application, Command, Element, Theme,
};
use miyoo_mini_hal::model::Model;
use once_cell::sync::Lazy;
use system::{games::GameCache, Init, Settings, SystemMessage};
use tokio::sync::mpsc;

use crate::{screens::Screen, Message};

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub battery_percentage: u8,
    /// Defaults to MiniPlus but is updated after startup
    pub model: Model,
    pub settings: Settings,
    pub games: GameCache,
    pub event_receiver: RefCell<Option<mpsc::Receiver<SystemMessage>>>,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (event_receiver, system_fut) = system::task();
        (
            Self {
                screen: Screen::default(),
                battery_percentage: 100,
                model: Model::MiniPlus,
                settings: Settings::default(),
                games: GameCache::new(),
                event_receiver: RefCell::new(Some(event_receiver)),
            },
            // Start the system task and fullscreen the app
            Command::perform(system_fut, |init| Message::StartupDone(init)),
        )
    }

    fn title(&self) -> String {
        "Oxide".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::System(SystemMessage::Shutdown) => {
                self.screen = Screen::Shutdown;
                Command::none()
            }
            Message::System(SystemMessage::Sleep) if matches!(self.screen, Screen::Sleep(_)) => {
                self.screen = Screen::Sleep(Some(Box::new(self.screen.clone())));
                Command::none()
            }
            Message::System(SystemMessage::MainMenu) => {
                self.screen = Screen::Main;
                Command::none()
            }
            Message::System(SystemMessage::Switcher) => {
                if matches!(self.screen, Screen::Switcher) {
                    self.screen = Screen::Main;
                } else {
                    self.screen = Screen::Switcher;
                }
                Command::none()
            }
            Message::StartupDone(Init {
                model,
                settings,
                games,
            }) => {
                let command = Command::single(Action::Window(window::Action::Resize {
                    width: model.width(),
                    height: model.height(),
                }));
                self.model = model;
                self.settings = settings;
                self.games = games;
                self.screen = Screen::Main;
                command
            }
            Message::System(SystemMessage::BatteryPercentage(percentage)) => {
                self.battery_percentage = percentage;
                Command::none()
            }
            _ => Screen::update(self, message),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        Screen::view(self)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::unfold(
            "Input Events",
            self.event_receiver.take(),
            move |mut event_receiver| async move {
                let event = event_receiver.as_mut().unwrap().recv().await.unwrap();
                tracing::debug!("Sys Message: {event:?}");
                (Message::System(event), event_receiver)
            },
        )
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
