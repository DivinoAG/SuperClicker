use iced::{executor, Application, Command, Element, Theme, Subscription};

use super::ui;
use super::clicking::ClickingEngine;
use super::hotkeys::{self, HotkeyEvent};
use super::settings::Settings;
use std::thread;
use std::time::{Duration, Instant};

pub struct SuperClicker {
    status: String,
    interval_input: String,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
    is_running: bool,
    clicking_engine: ClickingEngine,
    settings: Settings,
    last_toggle: Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    IntervalInputChanged(String),
    MouseButtonSelected(String),
    DynamicAdjustmentToggled(bool),
    Start,
    Stop,
    ToggleFromHotkey,
    NoOp,
}

impl Application for SuperClicker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let settings = Settings::load().unwrap_or_default();

        (
            SuperClicker {
                status: String::from("Stopped"),
                interval_input: settings.interval_ms.to_string(),
                mouse_button_selected: settings.mouse_button.clone(),
                enable_dynamic_adjustment: settings.enable_dynamic_adjustment,
                is_running: false,
                clicking_engine: ClickingEngine::new(),
                settings,
                last_toggle: Instant::now(),
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
                self.interval_input = value.clone();
                if let Ok(ms) = value.parse::<u64>() {
                    self.settings.interval_ms = ms;
                    let _ = self.settings.save();
                }
            }
            Message::MouseButtonSelected(value) => {
                self.mouse_button_selected = value.clone();
                self.settings.mouse_button = value;
                let _ = self.settings.save();
            }
            Message::DynamicAdjustmentToggled(value) => {
                self.enable_dynamic_adjustment = value;
                self.settings.enable_dynamic_adjustment = value;
                let _ = self.settings.save();
            }
            Message::Start => {
                if self.is_running {
                    self.clicking_engine.stop();
                    thread::sleep(Duration::from_millis(50));
                }
                self.is_running = true;
                self.status = String::from("Running");
                let interval_ms: u64 = self.interval_input.parse().unwrap_or(100);
                self.clicking_engine.start(&self.mouse_button_selected, interval_ms);
            }
            Message::Stop => {
                println!("[Stop] received");
                self.is_running = false;
                self.status = String::from("Stopped");
                self.clicking_engine.stop();
            }
            Message::ToggleFromHotkey => {
                // Debounce logic: prevent double-toggling if both rdev and iced see the event
                let now = Instant::now();
                if now.duration_since(self.last_toggle) < Duration::from_millis(300) {
                    return Command::none();
                }
                self.last_toggle = now;

                if self.is_running {
                    self.is_running = false;
                    self.status = String::from("Stopped");
                    self.clicking_engine.stop();
                } else {
                    self.is_running = true;
                    self.status = String::from("Running");
                    let interval_ms: u64 = self.interval_input.parse().unwrap_or(100);
                    self.clicking_engine.start(&self.mouse_button_selected, interval_ms);
                }
            }
            Message::NoOp => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            hotkeys::subscribe().map(|event| match event {
                HotkeyEvent::Toggle => Message::ToggleFromHotkey,
                HotkeyEvent::Ignore => Message::NoOp,
            }),
            hotkeys::subscribe_local().map(|event| match event {
                HotkeyEvent::Toggle => Message::ToggleFromHotkey,
                HotkeyEvent::Ignore => Message::NoOp,
            }),
        ])
    }

    fn view(&self) -> Element<'_, Self::Message> {
        ui::view(
            &self.status,
            &self.interval_input,
            self.mouse_button_selected.clone(),
            self.enable_dynamic_adjustment,
            self.is_running,
        )
    }
}