mod app;
mod ui;
mod clicking;
mod hotkeys;
mod settings;

use iced::{Application, Settings};
use app::SuperClicker;

fn main() -> iced::Result {
    SuperClicker::run(Settings::default())
}
