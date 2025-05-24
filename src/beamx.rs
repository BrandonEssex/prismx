use ratatui::{backend::Backend, layout::Rect, style::{Color, Style}, widgets::Paragraph, Frame};

pub struct BeamStyle {
    pub border_color: Color,
    pub status_color: Color,
    pub prism_color: Color,
    pub center_glyph: &'static str,
}

pub fn style_for_mode(mode: &str) -> BeamStyle {
    match mode {
        "gemx" => BeamStyle {
            border_color: Color::Cyan,
            status_color: Color::White,
            prism_color: Color::White,
            center_glyph: "✦",
        },
        "zen" => BeamStyle {
            border_color: Color::Green,
            status_color: Color::Gray,
            prism_color: Color::White,
            center_glyph: "✦",
        },
        "triage" => BeamStyle {
            border_color: Color::White,
            status_color: Color::Red,
            prism_color: Color::Cyan,
            center_glyph: "✦",
        },
        "settings" => BeamStyle {
            border_color: Color::Gray,
            status_color: Color::Gray,
            prism_color: Color::White,
            center_glyph: "✦",
        },
        _ => BeamStyle {
            border_color: Color::Gray,
            status_color: Color::Gray,
            prism_color: Color::Gray,
            center_glyph: "✦",
        },
    }
}

/// Render a beam logo using custom colors.
pub fn render_beam_logo<B: Backend>(f: &mut Frame<B>, area: Rect, style: &BeamStyle) {
    let x_offset = area.x + area.width.saturating_sub(6);
    let y_offset = area.y;

    let style_border = Style::default().fg(style.border_color);
    let style_status = Style::default().fg(style.status_color);
    let style_prism = Style::default().fg(style.prism_color);

    // Line 0
    let para = Paragraph::new("╱").style(style_border);
    f.render_widget(para, Rect::new(x_offset, y_offset, 1, 1));
    let para = Paragraph::new("/").style(style_status);
    f.render_widget(para, Rect::new(x_offset + 3, y_offset, 1, 1));

    // Line 1 (center glyph)
    let para = Paragraph::new(style.center_glyph).style(style_prism);
    f.render_widget(para, Rect::new(x_offset + 2, y_offset + 1, 1, 1));

    // Line 2
    let para = Paragraph::new("/").style(style_status);
    f.render_widget(para, Rect::new(x_offset, y_offset + 2, 1, 1));
    let para = Paragraph::new("╲").style(style_border);
    f.render_widget(para, Rect::new(x_offset + 3, y_offset + 2, 1, 1));
}

pub fn render_full_border<B: Backend>(f: &mut Frame<B>, area: Rect, style: &BeamStyle) {
    let fg = Style::default().fg(style.border_color);
    let right = area.x + area.width - 1;
    let bottom = area.y + area.height - 1;

    let tl = Paragraph::new("┏").style(fg);
    f.render_widget(tl, Rect::new(area.x, area.y, 1, 1));
    for x in area.x + 1..right {
        let p = Paragraph::new("━").style(fg);
        f.render_widget(p, Rect::new(x, area.y, 1, 1));
    }
    // Skip top-right corner so the beam can cut through

    for y in area.y + 1..bottom {
        let p = Paragraph::new("┃").style(fg);
        f.render_widget(p, Rect::new(area.x, y, 1, 1));
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


