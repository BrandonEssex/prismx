use std::collections::HashMap;
use std::fs;

use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PluginTheme {
    pub panel_fg: Option<String>,
    pub panel_bg: Option<String>,
    pub accent: Option<String>,
    pub icon_color: Option<String>,
}

pub fn load_plugin_themes(dir: &str) -> HashMap<String, PluginTheme> {
    let mut themes = HashMap::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path().join("theme.toml");
            if path.exists() {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(theme) = toml::from_str::<PluginTheme>(&data) {
                        if let Some(name) = entry.file_name().to_str() {
                            themes.insert(name.to_string(), theme);
                        }
                    }
                }
            }
        }
    }
    themes
}

pub fn parse_color(hex: &str) -> Option<Color> {
    let trimmed = hex.trim_start_matches('#');
    u32::from_str_radix(trimmed, 16)
        .ok()
        .map(|rgb| {
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            Color::Rgb(r, g, b)
        })
}
