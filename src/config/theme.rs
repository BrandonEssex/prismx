use serde::{Deserialize, Serialize};
use std::fs;
use ratatui::style::{Color, Style};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    pub dark_mode: bool,
    pub opacity: f32,
    pub zen_peaceful: bool,
    pub zen_breathe: bool,
    pub text_brightness: f32,
    pub border_contrast: f32,
    pub accent_color: String,
    pub dock_pulse: bool,
    pub layout_mode: Option<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            opacity: 1.0,
            zen_peaceful: false,
            zen_breathe: true,
            text_brightness: 1.0,
            border_contrast: 1.0,
            accent_color: "cyan".into(),
            dock_pulse: true,
            layout_mode: Some("tree".into()),
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

    /// Accent color used for highlights
    pub fn accent_color(&self) -> Color {
        if self.dark_mode {
            Color::LightCyan
        } else {
            Color::Blue
        }
    }

    /// Accent color used for focused selections.
    pub fn focus_outline(&self) -> Color {
        if self.dark_mode {
            Color::LightCyan
        } else {
            Color::Blue
        }
    }

    pub fn dock_pulse(&self) -> bool {
        self.dock_pulse
    }

    pub fn dim_color(&self) -> Color {
        if self.dark_mode { Color::DarkGray } else { Color::Gray }
    }

    pub fn layout_mode(&self) -> crate::gemx::layout::LayoutMode {
        match self.layout_mode.as_deref() {
            Some("grid") => crate::gemx::layout::LayoutMode::Grid,
            Some("hybrid") => crate::gemx::layout::LayoutMode::Hybrid,
            _ => crate::gemx::layout::LayoutMode::Tree,
        }
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
