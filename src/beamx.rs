use ratatui::{backend::Backend, layout::Rect, style::{Color, Style}, widgets::Paragraph, Frame};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BeamStyle {
    pub border_color: Color,
    pub status_color: Color,
    pub prism_color: Color,
    pub center_glyph: &'static str,
}

#[derive(Copy, Clone)]
pub enum BeamXStyle {
    /// Beam hits prism then splits out
    Split,
    /// Stylized X beam
    Cross,
}

pub fn style_for_mode(mode: &str) -> BeamStyle {
    match mode {
        "gemx" => BeamStyle {
            border_color: Color::Cyan,
            status_color: Color::White,
            prism_color: Color::White,
            center_glyph: "◆",
        },
        "zen" => BeamStyle {
            border_color: Color::Green,
            status_color: Color::Gray,
            prism_color: Color::White,
            center_glyph: "◆",
        },
        "triage" => BeamStyle {
            border_color: Color::White,
            status_color: Color::Red,
            prism_color: Color::Cyan,
            center_glyph: "◆",
        },
        "settings" => BeamStyle {
            border_color: Color::Gray,
            status_color: Color::Gray,
            prism_color: Color::White,
            center_glyph: "◆",
        },
        _ => BeamStyle {
            border_color: Color::Gray,
            status_color: Color::Gray,
            prism_color: Color::Gray,
            center_glyph: "◆",
        },
    }
}

/// Render a beam logo using custom colors.
pub fn render_beamx<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    style: &BeamStyle,
    variant: BeamXStyle,
) {
    let x_offset = area.right().saturating_sub(7);
    let y_offset = area.top();

    let style_border = Style::default().fg(style.border_color);
    let style_status = Style::default().fg(style.status_color);
    let style_prism = Style::default().fg(style.prism_color);

    // simple time-based tick for prism animation
    let tick = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300;
    let prism = match tick % 12 {
        0 => "·",
        1 => "✦",
        2 => "◆",
        3 => "✦",
        4 => "·",
        5 => "x",
        6 => "X",
        7 => "x",
        8 => "·",
        9 => "✦",
        10 => "◆",
        _ => "✦",
    };

    match variant {
        BeamXStyle::Split => {
            // top row corners
            let tl = Paragraph::new("⬊").style(style_border);
            f.render_widget(tl, Rect::new(x_offset, y_offset, 1, 1));
            let tr = Paragraph::new("⬋").style(style_border);
            f.render_widget(tr, Rect::new(x_offset + 4, y_offset, 1, 1));

            // center glyph
            let center_widget = Paragraph::new(prism).style(style_prism);
            f.render_widget(center_widget, Rect::new(x_offset + 2, y_offset + 1, 1, 1));

            // bottom row corners
            let bl = Paragraph::new("⬈").style(style_border);
            f.render_widget(bl, Rect::new(x_offset, y_offset + 2, 1, 1));
            let br = Paragraph::new("⬉").style(style_border);
            f.render_widget(br, Rect::new(x_offset + 4, y_offset + 2, 1, 1));
        }
        BeamXStyle::Cross => {
            let left = Paragraph::new("⨯").style(style_status);
            f.render_widget(left, Rect::new(x_offset + 1, y_offset + 1, 1, 1));
            let prism_widget = Paragraph::new(prism).style(style_prism);
            f.render_widget(prism_widget, Rect::new(x_offset + 2, y_offset + 1, 1, 1));
            let right = Paragraph::new("⨯").style(style_border);
            f.render_widget(right, Rect::new(x_offset + 3, y_offset + 1, 1, 1));
        }
    }
}

/// Legacy alias for default logo
pub fn render_beam_logo<B: Backend>(f: &mut Frame<B>, area: Rect, style: &BeamStyle) {
    render_beamx(f, area, style, BeamXStyle::Split);
}

pub fn render_full_border<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    style: &BeamStyle,
    beamx_enabled: bool,
) {
    let fg = Style::default().fg(style.border_color);
    let right = area.x + area.width - 1;
    let bottom = area.y + area.height - 1;

    let tl = Paragraph::new("┏").style(fg);
    f.render_widget(tl, Rect::new(area.x, area.y, 1, 1));
    let beam_start = area.right().saturating_sub(7);
    let beam_end = beam_start + 6;
    for x in area.x + 1..right {
        if beamx_enabled && x >= beam_start && x <= beam_end {
            continue;
        }
        let p = Paragraph::new("━").style(fg);
        f.render_widget(p, Rect::new(x, area.y, 1, 1));
    }
    if !beamx_enabled {
        let tr = Paragraph::new("┓").style(fg);
        f.render_widget(tr, Rect::new(right, area.y, 1, 1));
    }

    let beam_x = area.right() - 1;
    let beam_y1 = area.top() + 1;
    let beam_y2 = area.top() + 2;
    for y in area.y + 1..bottom {
        let p = Paragraph::new("┃").style(fg);
        f.render_widget(p, Rect::new(area.x, y, 1, 1));
        if beamx_enabled && right == beam_x && (y == beam_y1 || y == beam_y2) {
            continue;
        }
        let p2 = Paragraph::new("┃").style(fg);
        f.render_widget(p2, Rect::new(right, y, 1, 1));
    }

    let bl = Paragraph::new("┗").style(fg);
    f.render_widget(bl, Rect::new(area.x, bottom, 1, 1));
    for x in area.x + 1..right {
        let p = Paragraph::new("━").style(fg);
        f.render_widget(p, Rect::new(x, bottom, 1, 1));
    }
    let br = Paragraph::new("┛").style(fg);
    f.render_widget(br, Rect::new(right, bottom, 1, 1));
}


