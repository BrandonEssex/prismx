use ratatui::{backend::Backend, layout::Rect, style::{Color, Style}, widgets::Paragraph, Frame};

pub struct BeamStyle {
    pub border_color: Color,
    pub beam_left: Color,
    pub beam_right: Color,
    pub prism: Color,
}

pub fn style_for_mode(mode: &str) -> BeamStyle {
    match mode {
        "gemx" => BeamStyle {
            border_color: Color::Magenta,
            beam_left: Color::Cyan,
            beam_right: Color::White,
            prism: Color::White,
        },
        "zen" => BeamStyle {
            border_color: Color::White,
            beam_left: Color::Green,
            beam_right: Color::Green,
            prism: Color::White,
        },
        "triage" => BeamStyle {
            border_color: Color::Red,
            beam_left: Color::White,
            beam_right: Color::White,
            prism: Color::Cyan,
        },
        "settings" => BeamStyle {
            border_color: Color::Blue,
            beam_left: Color::Gray,
            beam_right: Color::Gray,
            prism: Color::White,
        },
        _ => BeamStyle {
            border_color: Color::Gray,
            beam_left: Color::Gray,
            beam_right: Color::Gray,
            prism: Color::Gray,
        },
    }
}

/// Render a beam logo using custom colors.
pub fn render_beam_logo<B: Backend>(f: &mut Frame<B>, area: Rect, style: &BeamStyle) {
    let x_offset = area.width.saturating_sub(6);
    let y_offset = area.y + 1;

    let style_left = Style::default().fg(style.beam_left);
    let style_right = Style::default().fg(style.beam_right);
    let style_prism = Style::default().fg(style.prism);

    // Line 0
    let para = Paragraph::new("\\").style(style_left);
    f.render_widget(para, Rect::new(x_offset, y_offset, 1, 1));
    let para = Paragraph::new("/").style(style_right);
    f.render_widget(para, Rect::new(x_offset + 3, y_offset, 1, 1));

    // Line 1 (prism center)
    let para = Paragraph::new("*").style(style_prism);
    f.render_widget(para, Rect::new(x_offset + 2, y_offset + 1, 1, 1));

    // Line 2
    let para = Paragraph::new("/").style(style_left);
    f.render_widget(para, Rect::new(x_offset, y_offset + 2, 1, 1));
    let para = Paragraph::new("\\").style(style_right);
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
    let tr = Paragraph::new("┓").style(fg);
    f.render_widget(tr, Rect::new(right, area.y, 1, 1));

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


