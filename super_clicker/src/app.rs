use iced::{executor, Application, Command, Element, Theme, Subscription};

use super::ui;
use super::clicking::ClickingEngine;
use super::hotkeys::HotkeyManager;
use super::settings::Settings;
use std::thread;
use std::time::Duration;

pub struct SuperClicker {
    status: String,
    interval_input: String,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
    is_running: bool,
    clicking_engine: ClickingEngine,
    hotkey_manager: HotkeyManager,
    settings: Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    IntervalInputChanged(String),
    MouseButtonSelected(String),
    DynamicAdjustmentToggled(bool),
    Start,
    Stop,
    ToggleFromHotkey,
    Tick,
}

impl Application for SuperClicker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let settings = Settings::load().unwrap_or_default();

        let hotkey_manager = HotkeyManager::new();
        // Start listening for hotkeys
        hotkey_manager.start_listening();

        (
            SuperClicker {
                status: String::from("Stopped"),
                interval_input: settings.interval_ms.to_string(),
                mouse_button_selected: settings.mouse_button.clone(),
                enable_dynamic_adjustment: settings.enable_dynamic_adjustment,
                is_running: false,
                clicking_engine: ClickingEngine::new(),
                hotkey_manager,
                settings,
            },
            Command::none(),
        )
    }    fn title(&self) -> String {
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
                        // Already running, stop first to clean up the old thread
                        self.clicking_engine.stop();
                        thread::sleep(Duration::from_millis(50)); // Give thread time to exit
                    }
                    self.is_running = true;
                    self.status = String::from("Running");
                    let interval_ms: u64 = self.interval_input.parse().unwrap_or(100);
                    self.clicking_engine.start(&self.mouse_button_selected, interval_ms);
                }
                Message::Stop => {
                    println!("[Stop] received - current is_running: {}", self.is_running);
                    self.is_running = false;
                    self.status = String::from("Stopped");
                    self.clicking_engine.stop();
                    println!("[Stop] Complete - is_running now: {}", self.is_running);
                }
                Message::ToggleFromHotkey => {
                    // Toggle the running state
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
                Message::Tick => {
                    if self.hotkey_manager.check_event() {
                        return self.update(Message::ToggleFromHotkey);
                    }
                }
            }

            Command::none()
        }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick)
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
