use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub interval_ms: u64,
    pub mouse_button: String,
    pub enable_dynamic_adjustment: bool,
    pub toggle_hotkey_ctrl: bool,
    pub toggle_hotkey_alt: bool,
    pub toggle_hotkey_shift: bool,
    pub toggle_hotkey_key: Option<String>,
    pub interval_hotkey_ctrl: bool,
    pub interval_hotkey_alt: bool,
    pub interval_hotkey_shift: bool,
    pub interval_hotkey_key: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            interval_ms: 100,
            mouse_button: String::from("Left"),
            enable_dynamic_adjustment: true,
            toggle_hotkey_ctrl: true,
            toggle_hotkey_alt: true,
            toggle_hotkey_shift: false,
            toggle_hotkey_key: Some(String::from("F6")),
            interval_hotkey_ctrl: true,
            interval_hotkey_alt: true,
            interval_hotkey_shift: true,
            interval_hotkey_key: None,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let settings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(Settings::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, json)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().ok_or("Could not determine config directory")?;
        Ok(config_dir.join("super_clicker").join("settings.json"))
    }
}
