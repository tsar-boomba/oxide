use std::cell::RefCell;

use iced::{executor, subscription, Application, Command, Element, Theme, widget::text, color};
use system::SystemMessage;
use tokio::sync::mpsc;

use crate::{
    screens::Screen,
    Message,
};

#[derive(Debug)]
pub struct App {
    screen: Screen,
    event_receiver: RefCell<Option<mpsc::Receiver<SystemMessage>>>,
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
                event_receiver: RefCell::new(Some(event_receiver)),
            },
            Command::perform(system_fut, |_| Message::Noop),
        )
    }

    fn title(&self) -> String {
        "Oxide".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.screen.update(message)
    }

    fn view(&self) -> Element<Self::Message> {
        text("hello world").size(20).style(color!(0, 0, 255)).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::unfold(
            "Input Events",
            self.event_receiver.take(),
            move |mut event_receiver| async move {
                let event = event_receiver.as_mut().unwrap().recv().await.unwrap();
                tracing::info!("Ui recv ev: {event:?}");
                (Message::System(event), event_receiver)
            },
        )
    }
}
