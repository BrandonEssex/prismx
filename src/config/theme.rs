use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeConfig {
    pub dark_mode: bool,
    pub opacity: f32,
    pub zen_peaceful: bool,
    pub zen_breathe: bool,
    pub layout_mode: Option<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            opacity: 1.0,
            zen_peaceful: false,
            zen_breathe: false,
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

    pub fn zen_peaceful(&self) -> bool {
        self.zen_peaceful
    }

    pub fn zen_breathe(&self) -> bool {
        self.zen_breathe
    }

    pub fn layout_mode(&self) -> crate::gemx::layout::LayoutMode {
        match self.layout_mode.as_deref() {
            Some("grid") => crate::gemx::layout::LayoutMode::Grid,
            Some("hybrid") => crate::gemx::layout::LayoutMode::Hybrid,
            _ => crate::gemx::layout::LayoutMode::Tree,
        }
    }
}
