mod application;

use iced::{Size, window::Settings};

use application::CheckersApplication;

fn main() -> iced::Result {
    iced::application(
        "The Checkers Game",
        CheckersApplication::update,
        CheckersApplication::view,
    )
    .window(Settings {
        size: Size {
            width: 300.0,
            height: 300.0,
        },
        ..Settings::default()
    })
    .run()
}
