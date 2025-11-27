use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, StreamExt};
use iced::Subscription;
use iced::keyboard;
use iced::Event;
use iced::event;
use rdev::{listen, EventType, Key};
use std::thread;

#[derive(Debug, Clone)]
pub enum HotkeyEvent {
    Toggle,
    Ignore,
}

pub fn subscribe() -> Subscription<HotkeyEvent> {
    iced::subscription::channel(
        std::any::TypeId::of::<HotkeyEvent>(),
        100,
        |mut output| async move {
            let (mut sender, mut receiver) = mpsc::channel(100);

            thread::spawn(move || {
                println!("Starting rdev listener thread...");
                let mut ctrl = false;
                let mut alt = false;

                // Listen loop. This blocks the thread.
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
                        _ => {}
                    }
                }) {
                    eprintln!("Error in rdev::listen: {:?}", error);
                }
            });

            while let Some(event) = receiver.next().await {
                let _ = output.send(event).await;
            }
            
            // The subscription channel expects a Future that never resolves.
            loop {
                std::future::pending::<()>().await;
            }
        },
    )
}

pub fn subscribe_local() -> Subscription<HotkeyEvent> {
    event::listen().map(|event| {
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modifiers,
            ..
        }) = event {
            if key == keyboard::Key::Named(keyboard::key::Named::F6) 
               && modifiers.control() 
               && modifiers.alt() 
            {
                return HotkeyEvent::Toggle;
            }
        }
        HotkeyEvent::Ignore
    })
}
