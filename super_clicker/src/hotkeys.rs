use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, StreamExt};
use iced::Subscription;
use iced::keyboard;
use iced::mouse;
use iced::Event;
use iced::event;
use rdev::{listen, EventType, Key};
use std::thread;

#[derive(Debug, Clone)]
pub enum HotkeyEvent {
    Toggle,
    IntervalChange(i32), // From rdev (Trusted)
    LocalScroll(f32),    // From iced (Untrusted, need to check modifiers)
    ModifiersChanged(bool, bool), // From iced
    Ignore,
}

pub fn subscribe() -> Subscription<HotkeyEvent> {
    iced::subscription::channel(
        std::any::TypeId::of::<HotkeyEvent>(),
        100,
        |mut output| async move {
            let (mut sender, mut receiver) = mpsc::channel(100);

            thread::spawn(move || {
                // Listen loop. This blocks the thread.
                let mut ctrl = false;
                let mut alt = false;
                
                if let Err(error) = listen(move |event| {
                    match event.event_type {
                        EventType::KeyPress(Key::ControlLeft) | EventType::KeyPress(Key::ControlRight) => {
                            ctrl = true;
                        }
                        EventType::KeyRelease(Key::ControlLeft) | EventType::KeyRelease(Key::ControlRight) => {
                            ctrl = false;
                        }
                        EventType::KeyPress(Key::Alt) | EventType::KeyPress(Key::AltGr) => {
                            alt = true;
                        }
                        EventType::KeyRelease(Key::Alt) | EventType::KeyRelease(Key::AltGr) => {
                            alt = false;
                        }
                        EventType::KeyPress(Key::F6) => {
                            if ctrl && alt {
                                let _ = sender.try_send(HotkeyEvent::Toggle);
                            }
                        }
                        EventType::Wheel { delta_y, .. } => {
                            if ctrl && alt {
                                // Send raw delta
                                let _ = sender.try_send(HotkeyEvent::IntervalChange(delta_y as i32));
                            }
                        }
                        _ => {}
                    }
                }) {
                    eprintln!("Error in rdev::listen: {:?}", error);
                }
            });

            while let Some(event) = receiver.next().await {
                let _ = output.send(event).await;
            }
            
            loop {
                std::future::pending::<()>().await;
            }
        },
    )
}

pub fn subscribe_local() -> Subscription<HotkeyEvent> {
    event::listen().map(|event| {
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed { modifiers, key, .. }) => {
                 if key == keyboard::Key::Named(keyboard::key::Named::F6) 
                    && modifiers.control() && modifiers.alt() {
                     return HotkeyEvent::Toggle;
                 }
                 return HotkeyEvent::ModifiersChanged(modifiers.control(), modifiers.alt());
            }
            Event::Keyboard(keyboard::Event::KeyReleased { modifiers, .. }) => {
                 return HotkeyEvent::ModifiersChanged(modifiers.control(), modifiers.alt());
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                match delta {
                    mouse::ScrollDelta::Lines { y, .. } => return HotkeyEvent::LocalScroll(y),
                    mouse::ScrollDelta::Pixels { y, .. } => return HotkeyEvent::LocalScroll(y),
                }
            }
            _ => HotkeyEvent::Ignore
        }
    })
}