use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeConfig {
    pub dark_mode: bool,
    pub opacity: f32,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            opacity: 1.0,
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
}
