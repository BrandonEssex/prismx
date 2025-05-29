use ratatui::style::{Style, Modifier};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self { FontStyle::Regular }
}

impl FontStyle {
    pub fn next(self) -> Self {
        match self {
            FontStyle::Regular => FontStyle::Bold,
            FontStyle::Bold => FontStyle::Italic,
            FontStyle::Italic => FontStyle::Regular,
        }
    }

    pub fn style(self) -> Style {
        match self {
            FontStyle::Regular => Style::default(),
            FontStyle::Bold => Style::default().add_modifier(Modifier::BOLD),
            FontStyle::Italic => Style::default().add_modifier(Modifier::ITALIC),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            FontStyle::Regular => "Regular",
            FontStyle::Bold => "Bold",
            FontStyle::Italic => "Italic",
        }
    }
}
