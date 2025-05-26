use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    pub dark_mode: bool,
    pub opacity: f32,
    pub zen_peaceful: bool,
    pub zen_breathe: bool,
    pub text_brightness: f32,
    pub border_contrast: f32,
    pub accent_color: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            opacity: 1.0,
            zen_peaceful: false,
            zen_breathe: false,
            text_brightness: 1.0,
            border_contrast: 1.0,
            accent_color: "cyan".into(),
        }
    }
}

impl ThemeConfig {
    pub fn load() -> Self {
        fs::read_to_string("config/theme.toml")
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// Save the current theme configuration to `config/theme.toml`.
    pub fn save(&self) {
        if let Ok(serialized) = toml::to_string(self) {
            let _ = fs::create_dir_all("config");
            let _ = fs::write("config/theme.toml", serialized);
        }
    }

    pub fn zen_peaceful(&self) -> bool {
        self.zen_peaceful
    }

    pub fn zen_breathe(&self) -> bool {
        self.zen_breathe
    }
}
