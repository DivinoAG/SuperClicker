mod app;
mod ui;
mod clicking;
mod hotkeys;
mod settings;

use iced::{Application, Settings};
use app::SuperClicker;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = iced::Size::new(400.0, 350.0);
    settings.window.resizable = false;
    SuperClicker::run(settings)
}