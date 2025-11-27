use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager, hotkey::Modifiers};
use std::sync::{Arc, Mutex, mpsc};

#[derive(Clone)]
pub struct HotkeyManager {
    hotkey_manager: Arc<Mutex<Option<GlobalHotKeyManager>>>,
    hotkey_registered: Arc<Mutex<bool>>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        match GlobalHotKeyManager::new() {
            Ok(manager) => {
                HotkeyManager {
                    hotkey_manager: Arc::new(Mutex::new(Some(manager))),
                    hotkey_registered: Arc::new(Mutex::new(false)),
                }
            }
            Err(e) => {
                eprintln!("Failed to create GlobalHotKeyManager: {}", e);
                HotkeyManager {
                    hotkey_manager: Arc::new(Mutex::new(None)),
                    hotkey_registered: Arc::new(Mutex::new(false)),
                }
            }
        }
    }

    pub fn start_listening(&self, tx: mpsc::Sender<()>) {
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
                        *self.hotkey_registered.lock().unwrap() = true;

                    }
                    Err(e) => {
                        eprintln!("Failed to register hotkey: {}", e);
                    }
                }
            }
        }
    }

    pub fn stop_listening(&self) {
        *self.hotkey_registered.lock().unwrap() = false;
    }
}
