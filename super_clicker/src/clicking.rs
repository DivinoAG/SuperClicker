use rdev::{simulate, Button};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct ClickingEngine {
    is_running: Arc<Mutex<bool>>,
}

impl ClickingEngine {
    pub fn new() -> Self {
        ClickingEngine {
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self, button: &str, interval_ms: u64) {
        let is_running = Arc::clone(&self.is_running);
        let button_clone = button.to_string();

        // Set running state to true
        *is_running.lock().unwrap() = true;

        thread::spawn(move || {
            loop {
                let start_time = Instant::now();

                {
                    let running = is_running.lock().unwrap();
                    if !*running {
                        break;
                    }
                }

                let btn = match button_clone.as_str() {
                    "Left" => Button::Left,
                    "Right" => Button::Right,
                    "Middle" => Button::Middle,
                    _ => Button::Left,
                };

                // Simulate mouse down and up for a click
                let _ = simulate(&rdev::EventType::ButtonPress(btn));
                thread::sleep(Duration::from_millis(10));
                let _ = simulate(&rdev::EventType::ButtonRelease(btn));

                // Wait until the total interval_ms has elapsed since the start of this click
                let elapsed = start_time.elapsed().as_millis() as u64;
                if elapsed < interval_ms {
                    let wait_ms = interval_ms - elapsed;
                    thread::sleep(Duration::from_millis(wait_ms));
                }
            }
        });
    }

    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}
