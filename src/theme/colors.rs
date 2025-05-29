use ratatui::style::Color;

pub struct SpotlightPalette {
    pub background: Color,
    pub foreground: Color,
    pub active_background: Color,
}

impl SpotlightPalette {
    pub fn for_mode(dark: bool) -> Self {
        if dark {
            Self {
                background: Color::Rgb(30, 30, 40),
                foreground: Color::White,
                active_background: Color::Rgb(90, 90, 110),
            }
        } else {
            Self {
                background: Color::Rgb(230, 230, 235),
                foreground: Color::Black,
                active_background: Color::Rgb(200, 200, 210),
            }
        }
    }
}

/// Default background colors for sticky notes in the Triage panel.
pub const STICKY_COLORS: [Color; 4] = [
    Color::Rgb(252, 234, 170), // yellow
    Color::Rgb(250, 210, 180), // peach
    Color::Rgb(210, 240, 180), // green
    Color::Rgb(200, 220, 250), // blue
];
