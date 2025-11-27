use iced::{executor, Application, Command, Element, Theme};

use super::ui;
use super::clicking::ClickingEngine;

pub struct SuperClicker {
    status: String,
    interval_input: String,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
    is_running: bool,
    clicking_engine: ClickingEngine,
}

#[derive(Debug, Clone)]
pub enum Message {
    IntervalInputChanged(String),
    MouseButtonSelected(String),
    DynamicAdjustmentToggled(bool),
    Start,
    Stop,
}

impl Application for SuperClicker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            SuperClicker {
                status: String::from("Stopped"),
                interval_input: String::from("100"),
                mouse_button_selected: String::from("Left"),
                enable_dynamic_adjustment: true,
                is_running: false,
                clicking_engine: ClickingEngine::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SuperClicker")
    }

        fn update(
            &mut self,
            message: Self::Message,
        ) -> Command<Self::Message> {
            match message {
                Message::IntervalInputChanged(value) => {
                    self.interval_input = value;
                }
                Message::MouseButtonSelected(value) => {
                    self.mouse_button_selected = value;
                }
                Message::DynamicAdjustmentToggled(value) => {
                    self.enable_dynamic_adjustment = value;
                }
                Message::Start => {
                    self.is_running = true;
                    self.status = String::from("Running");
                    let interval_ms: u64 = self.interval_input.parse().unwrap_or(100);
                    self.clicking_engine.start(&self.mouse_button_selected, interval_ms);
                }
                Message::Stop => {
                    self.is_running = false;
                    self.status = String::from("Stopped");
                    self.clicking_engine.stop();
                }
            }
            Command::none()
        }

    fn view(&self) -> Element<Self::Message> {
        ui::view(
            &self.status,
            &self.interval_input,
            self.mouse_button_selected.clone(),
            self.enable_dynamic_adjustment,
        )
    }
}
