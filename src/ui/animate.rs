use ratatui::style::{Color, Style};

/// Cycle through a pulse sequence based on the provided tick.
pub fn pulse<'a>(frames: &'a [&'a str], tick: u64) -> &'a str {
    if frames.is_empty() {
        return "";
    }
    let i = tick as usize % frames.len();
    frames[i]
}

/// Simple shimmer effect that brightens a color every other tick.
pub fn shimmer(color: Color, tick: u64) -> Style {
    let style = Style::default().fg(color);
    if tick % 2 == 0 {
        style
    } else {
        style.add_modifier(ratatui::style::Modifier::BOLD)
    }
}

/// Oscillate a value between 0.0 and 1.0 producing a breathing effect.
pub fn breath(tick: u64) -> f32 {
    let phase = (tick % 20) as f32 / 20.0;
    if phase < 0.5 {
        phase * 2.0
    } else {
        (1.0 - phase) * 2.0
    }
}
