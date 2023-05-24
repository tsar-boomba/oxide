use iced::{Command, Element};

use crate::Message;

pub mod main;

#[derive(Debug)]
pub enum Screen {
	Main(main::State),
	Favorites,
	Games,
	Settings,
}

impl Screen {
	pub fn update(&mut self, message: Message) -> Command<Message> {
		match self {
            Self::Main(state) => state.handle_message(message),
            Self::Favorites => todo!(),
            Self::Games => todo!(),
            Self::Settings => todo!(),
        }
	}

	pub fn view(&self) -> Element<Message> {
		match self {
			Self::Main(state) => state.view(),
            Self::Favorites => todo!(),
            Self::Games => todo!(),
            Self::Settings => todo!(),
		}
	}
}

impl Default for Screen {
	fn default() -> Self {
		Self::Main(Default::default())
	}
}
