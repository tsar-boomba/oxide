use iced::{widget::{row, text, Text}, Command, Element, color};

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
        row![text("hello miyoo mini").size(20).style(color!(0, 0, 255))].padding(100).into()
    }
}
