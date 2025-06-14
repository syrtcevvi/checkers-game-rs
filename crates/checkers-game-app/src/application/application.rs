use iced::{
    Element,
    Length::Fill,
    widget::{container, text},
};

use crate::application::enums::Message;

#[derive(Debug, Default)]
pub struct CheckersApplication {}

impl CheckersApplication {
    pub fn view(&self) -> Element<Message> {
        container(text("Hello, world"))
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }
}
