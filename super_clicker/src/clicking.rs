use rdev::{simulate, Button};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

pub struct ClickingEngine {
    is_running: Arc<AtomicBool>,
    interval: Arc<AtomicU64>,
}

impl ClickingEngine {
    pub fn new() -> Self {
        ClickingEngine {
            is_running: Arc::new(AtomicBool::new(false)),
            interval: Arc::new(AtomicU64::new(100)),
        }
    }

    pub fn update_interval(&self, new_ms: u64) {
        self.interval.store(new_ms, Ordering::Relaxed);
    }

    pub fn start(&self, button: &str, interval_ms: u64) {
        let is_running = Arc::clone(&self.is_running);
        let interval = Arc::clone(&self.interval);
        let button_clone = button.to_string();

        // Set interval
        interval.store(interval_ms, Ordering::Relaxed);
        
        // Set running state to true
        is_running.store(true, Ordering::Relaxed);

        thread::spawn(move || {
            loop {
                let start_time = Instant::now();

                if !is_running.load(Ordering::Relaxed) {
                    break;
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

                // Read current interval dynamically
                let current_interval = interval.load(Ordering::Relaxed);

                // Wait until the total interval_ms has elapsed since the start of this click
                let elapsed = start_time.elapsed().as_millis() as u64;
                if elapsed < current_interval {
                    let wait_ms = current_interval - elapsed;
                    thread::sleep(Duration::from_millis(wait_ms));
                }
            }
        });
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}