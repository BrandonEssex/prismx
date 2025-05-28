use ratatui::style::Color;

pub struct ZenPalette {
    pub background: Color,
    pub text: Color,
    pub border: Color,
}

/// Return the peaceful Zen color palette.
/// Background is a soft midnight blue, text is pale yellow, borders are subtle.
pub fn zen_theme() -> ZenPalette {
    ZenPalette {
        background: Color::Rgb(20, 24, 36),
        text: Color::Rgb(240, 232, 190),
        border: Color::Rgb(30, 34, 48),
    }
}
