use evdev::{InputEvent, InputEventKind};

use crate::Button;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonEvent {
    button: Button,
    value: EventValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum EventValue {
    Released = 0,
    Pressed = 1,
    /// Currently unused
    Held = 2,
}

impl ButtonEvent {
    pub fn new(button: Button, value: EventValue) -> Self {
        Self { button, value }
    }

    /// Attempts to construct a ButtonEvent from any InputEvent
    pub fn from_event(event: InputEvent) -> Option<Self> {
        match event.kind() {
            InputEventKind::Key(key) => {
                let button = Button::from_key(key)?;
                let value = match event.value() {
                    0 => Some(EventValue::Released),
                    1 => Some(EventValue::Pressed),
                    _ => None,
                }?;

                Some(Self { button, value })
            }
            _ => None,
        }
    }

    pub fn from_raw(key_code: u16, value: EventValue) -> Option<Self> {
        let button = Button::from_raw(key_code)?;
        Some(Self { button, value })
    }

    #[inline(always)]
    pub fn button(&self) -> &Button {
        &self.button
    }

    #[inline(always)]
    pub fn value(&self) -> &EventValue {
        &self.value
    }

    #[inline(always)]
    pub fn pressed(&self) -> bool {
        self.value == EventValue::Pressed
    }

    #[inline(always)]
    pub fn released(&self) -> bool {
        self.value == EventValue::Released
    }

    #[inline(always)]
    pub fn held(&self) -> bool {
        self.value == EventValue::Held
    }

    #[inline(always)]
    pub fn is_pressed(&self, target: Button) -> bool {
        self.button == target && self.pressed()
    }

    #[inline(always)]
    pub fn is_released(&self, target: Button) -> bool {
        self.button == target && self.released()
    }

    #[inline(always)]
    pub fn is_held(&self, target: Button) -> bool {
        self.button == target && self.held()
    }
}
