use evdev::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    A,
    B,
    X,
    Y,
    L1,
    L2,
    R1,
    R2,
    Menu,
    Power,
    VolUp,
    VolDown,
}

impl Button {
	pub fn from_key(key: Key) -> Option<Self> {
		match key {
			Key::KEY_ESC => Some(Button::Menu),
			Key::KEY_POWER => Some(Button::Power),
			Key::KEY_RIGHTCTRL => Some(Button::Select),
			Key::KEY_ENTER => Some(Button::Start),
			Key::KEY_E => Some(Button::L1),
			Key::KEY_T => Some(Button::R1),
			Key::KEY_TAB => Some(Button::L2),
			Key::KEY_BACKSPACE => Some(Button::R2),
			Key::KEY_VOLUMEUP => Some(Button::VolUp),
			Key::KEY_VOLUMEDOWN => Some(Button::VolDown),
			Key::KEY_SPACE => Some(Button::A),
			Key::KEY_LEFTCTRL => Some(Button::B),
			Key::KEY_LEFTSHIFT => Some(Button::X),
			Key::KEY_LEFTALT => Some(Button::Y),
			_ => None
		}
	}

	pub fn from_raw(key_code: u16) -> Option<Self> {
		Self::from_key(Key::new(key_code))
	}
}
