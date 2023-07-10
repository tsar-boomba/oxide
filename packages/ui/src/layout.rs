use iced::{
    color,
    widget::{column, row, text},
    Element, Length,
};

use crate::{app::App, Message};

pub fn layout<'a>(app: &App, ui: Element<'a, Message>) -> Element<'a, Message> {
    column![
        row![
            text("Oxide").size(32).style(color!(0xF74C00)),
            text(format!("{}%", app.battery_percentage))
        ],
        ui
    ]
    .width(Length::Fill)
    .into()
}
