use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Clone)]
pub enum BeamDirection {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone)]
pub struct BeamSegment {
    pub color: Color,
    pub priority: u8,
    pub pulse: bool,
}

impl BeamSegment {
    pub fn color_name(&self) -> &'static str {
        match self.color {
            Color::Red => "R",
            Color::Green => "G",
            Color::Blue => "B",
            Color::Yellow => "Y",
            Color::Cyan => "C",
            Color::Magenta => "M",
            Color::White => "W",
            _ => "X"
        }
    }
}

#[derive(Clone, Default)]
pub struct BeamState {
    pub top_left: BeamSegment,
    pub top_right: BeamSegment,
    pub bottom_left: BeamSegment,
    pub bottom_right: BeamSegment,
}

#[derive(Default)]
pub struct IconProfile {
    pub name: String,
    pub beam_colors: [Color; 4],
    pub shard_color_map: HashMap<String, Color>,
    pub pulse_enabled: bool,
}