use iced::{widget::{row, text}, Command, Element};

use crate::Message;

#[derive(Debug)]
pub struct State {}

impl Default for State {
    fn default() -> Self {
        Self {  }
    }
}

impl State {
    pub fn handle_message(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        row![text("Some text :D")].into()
    }
}
