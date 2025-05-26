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

/// Return a style that dims and brightens with a breathing rhythm.
pub fn breath_style(color: Color, tick: u64) -> Style {
    if tick % 20 < 10 {
        Style::default().fg(color).add_modifier(ratatui::style::Modifier::DIM)
    } else {
        Style::default().fg(color)
    }
}

/// Return the cursor glyph for a simple blink animation.
pub fn cursor_blink(tick: u64) -> &'static str {
    if tick % 2 == 0 { "|" } else { " " }
}

/// Style that fades a cursor in and out using the breathing rhythm.
pub fn cursor_fade(tick: u64) -> Style {
    breath_style(Color::White, tick)
}

/// Scale factor for a short bounce animation.
/// `tick` should increment each frame up to 3.
pub fn soft_bounce(tick: u8, closing: bool) -> f32 {
    match tick {
        0 => {
            if closing {
                1.0
            } else {
                0.9
            }
        }
        1 => 1.05,
        2 => {
            if closing {
                0.9
            } else {
                1.0
            }
        }
        _ => 1.0,
    }
}
