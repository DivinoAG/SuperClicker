use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager, hotkey::Modifiers, GlobalHotKeyEvent, HotKeyState};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HotkeyManager {
    hotkey_manager: Arc<Mutex<Option<GlobalHotKeyManager>>>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        match GlobalHotKeyManager::new() {
            Ok(manager) => {
                HotkeyManager {
                    hotkey_manager: Arc::new(Mutex::new(Some(manager))),
                }
            }
            Err(e) => {
                eprintln!("Failed to create GlobalHotKeyManager: {}", e);
                HotkeyManager {
                    hotkey_manager: Arc::new(Mutex::new(None)),
                }
            }
        }
    }

    pub fn start_listening(&self) {
        // Create hotkey: Ctrl+Alt+F6
        let hk = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::ALT),
            "F6".parse().expect("Invalid key")
        );

        if let Ok(mut manager_opt) = self.hotkey_manager.lock() {
            if let Some(ref mut manager) = *manager_opt {
                match manager.register(hk) {
                    Ok(_id) => {
                        println!("Successfully registered hotkey: Ctrl+Alt+F6");
                    }
                    Err(e) => {
                        eprintln!("Failed to register hotkey: {}", e);
                    }
                }
            }
        }
    }

    pub fn check_event(&self) -> bool {
        let mut received = false;
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == HotKeyState::Released {
                received = true;
            }
        }
        received
    }
}
