use iced::{executor, Application, Command, Element, Subscription};

use super::ui;
use super::clicking::ClickingEngine;
use super::hotkeys::{self, HotkeyEvent};
use super::settings::Settings;
use super::theme::AppTheme;
use std::thread;
use std::time::{Duration, Instant};
use dark_light;

pub struct SuperClicker {
    status: String,
    interval_input: String,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
    is_running: bool,
    clicking_engine: ClickingEngine,
    settings: Settings,
    last_toggle: Instant,
    // Local state for foreground hotkeys
    ctrl_pressed: bool,
    alt_pressed: bool,
    last_scroll_time: Instant,
    current_theme: AppTheme, // Changed type to AppTheme
}

#[derive(Debug, Clone)]
pub enum Message {
    IntervalInputChanged(String),
    MouseButtonSelected(String),
    DynamicAdjustmentToggled(bool),
    Start,
    Stop,
    ToggleFromHotkey,
    IntervalChange(i32), // From rdev
    LocalScroll(f32),    // From iced
    ModifiersChanged(bool, bool),
    NoOp,
    CheckTheme,
    ThemeChanged(AppTheme), // Changed to AppTheme
}

impl Application for SuperClicker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme; // Keep iced::Theme as the associated type for Application
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let settings = Settings::load().unwrap_or_default();
        
        let mode = dark_light::detect();
        let theme = match mode {
            dark_light::Mode::Dark => AppTheme::Dark, // Initialize with AppTheme
            dark_light::Mode::Light => AppTheme::Light, // Initialize with AppTheme
            _ => AppTheme::Dark, // Default to Dark AppTheme
        };

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
                ctrl_pressed: false,
                alt_pressed: false,
                last_scroll_time: Instant::now(),
                current_theme: theme,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SuperClicker")
    }

    fn theme(&self) -> iced::Theme { // Returns iced::Theme
        self.current_theme.into() // Use into() to convert AppTheme to iced::Theme
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
                    self.clicking_engine.update_interval(ms);
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
            Message::IntervalChange(delta) => {
                self.adjust_interval(delta);
            }
            Message::ModifiersChanged(ctrl, alt) => {
                self.ctrl_pressed = ctrl;
                self.alt_pressed = alt;
            }
            Message::LocalScroll(delta_f32) => {
                if self.ctrl_pressed && self.alt_pressed {
                    self.adjust_interval(delta_f32 as i32);
                }
            }
            Message::NoOp => {}
            Message::CheckTheme => {
                let mode = dark_light::detect();
                let new_theme = match mode {
                    dark_light::Mode::Dark => AppTheme::Dark, // Pass AppTheme
                    dark_light::Mode::Light => AppTheme::Light, // Pass AppTheme
                    _ => AppTheme::Dark,
                };
                if new_theme != self.current_theme {
                    return Command::perform(async move { new_theme }, Message::ThemeChanged);
                }
            }
            Message::ThemeChanged(theme) => {
                self.current_theme = theme; // Set AppTheme
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            hotkeys::subscribe().map(|event| match event {
                HotkeyEvent::Toggle => Message::ToggleFromHotkey,
                HotkeyEvent::IntervalChange(delta) => Message::IntervalChange(delta),
                _ => Message::NoOp,
            }),
            hotkeys::subscribe_local().map(|event| match event {
                HotkeyEvent::Toggle => Message::ToggleFromHotkey,
                HotkeyEvent::ModifiersChanged(c, a) => Message::ModifiersChanged(c, a),
                HotkeyEvent::LocalScroll(d) => Message::LocalScroll(d),
                _ => Message::NoOp,
            }),
            iced::time::every(Duration::from_secs(1)).map(|_| Message::CheckTheme),
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

impl SuperClicker {
    fn adjust_interval(&mut self, delta: i32) {
        if !self.enable_dynamic_adjustment { return; }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_scroll_time);
        self.last_scroll_time = now;

        // Acceleration logic
        let mut multiplier = 1;
        // If consecutive scrolls happen within 50ms, apply heavy acceleration
        if elapsed < Duration::from_millis(50) {
            multiplier = 5;
        } else if elapsed < Duration::from_millis(100) {
            multiplier = 2;
        }

        let current_ms = self.interval_input.parse::<u64>().unwrap_or(100);
        
        let base_change = if delta.abs() < 10 {
            delta // 1 for 1ms adjustment
        } else {
            delta / 120 // 120 / 120 = 1ms adjustment
        };
        
        let change = base_change * multiplier;
        
        let new_ms_signed = (current_ms as i64) - (change as i64);
        let new_ms = new_ms_signed.max(1).min(10000) as u64;

        if new_ms != current_ms {
            self.interval_input = new_ms.to_string();
            self.settings.interval_ms = new_ms;
            let _ = self.settings.save();
            self.clicking_engine.update_interval(new_ms);
        }
    }
}