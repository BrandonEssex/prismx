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

/// Scale an RGB color by the provided ratio (0.0 - 1.0).
pub fn scale_color(color: Color, ratio: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb((r as f32 * ratio) as u8, (g as f32 * ratio) as u8, (b as f32 * ratio) as u8),
        Color::White => {
            let v = (255.0 * ratio) as u8;
            Color::Rgb(v, v, v)
        }
        Color::Gray => {
            let v = (128.0 * ratio) as u8;
            Color::Rgb(v, v, v)
        }
        Color::DarkGray => {
            let v = (105.0 * ratio) as u8;
            Color::Rgb(v, v, v)
        }
        Color::Blue => Color::Rgb(0, 0, (255.0 * ratio) as u8),
        Color::Magenta => Color::Rgb((255.0 * ratio) as u8, 0, (255.0 * ratio) as u8),
        Color::Red => Color::Rgb((255.0 * ratio) as u8, 0, 0),
        Color::Green => Color::Rgb(0, (255.0 * ratio) as u8, 0),
        Color::Yellow => Color::Rgb((255.0 * ratio) as u8, (255.0 * ratio) as u8, 0),
        _ => color,
    }
}

/// Patch a line's style with a fade-in effect based on the entry age.
pub fn fade_line(line: &mut ratatui::text::Line<'_>, age_ms: u128, duration_ms: u128) {
    let ratio = (age_ms.min(duration_ms) as f32) / (duration_ms as f32);
    for span in &mut line.spans {
        let fg = span.style.fg.unwrap_or(Color::White);
        span.patch_style(Style::default().fg(scale_color(fg, ratio)));
    }
}
