use serde::Deserialize;
use std::fs;
use ratatui::style::{Color, Style};

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeConfig {
    pub dark_mode: bool,
    pub opacity: f32,
    pub zen_peaceful: bool,
    pub zen_breathe: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            opacity: 1.0,
            zen_peaceful: false,
            zen_breathe: false,
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

    pub fn zen_peaceful(&self) -> bool {
        self.zen_peaceful
    }

    pub fn zen_breathe(&self) -> bool {
        self.zen_breathe
    }

    /// Style used to highlight selected rows with good contrast
    pub fn highlight_style(&self) -> Style {
        if self.dark_mode {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        }
    }

    /// Return a readable foreground color based on theme mode
    pub fn input_fg(&self) -> Color {
        if self.dark_mode { Color::White } else { Color::Black }
    }
}
