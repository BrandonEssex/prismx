use ratatui::{backend::Backend, layout::Rect, style::{Color, Style, Modifier}, widgets::Paragraph, Frame};

use super::beamx::{BeamXStyle, bright_color, render_glyph};

/// Draw the BeamX emblem with directional arrows and animations.
pub fn draw_beam<B: Backend>(f: &mut Frame<B>, area: Rect, tick: u64, style: &BeamXStyle) {
    if area.width < 12 || area.height < 5 {
        return; // Avoid drawing over small panels
    }

    let frame = tick % 12;
    let beam_phase = frame % 4;
    let center_phase = tick % 8;
    let x = area.right().saturating_sub(12);
    let y = area.top();

    let border = Style::default().fg(style.border_color);
    let status = Style::default().fg(style.status_color);

    let entry_glyph = "⇙";
    let exit_tl = "⬉";
    let exit_br = "⬊";

    let bright_status = Style::default()
        .fg(bright_color(style.status_color))
        .add_modifier(Modifier::BOLD);
    let entry_style = match beam_phase {
        0 => status.add_modifier(Modifier::DIM),
        1 => status,
        2 => status.add_modifier(Modifier::BOLD),
        _ => bright_status,
    };

    let bright_border = Style::default().fg(bright_color(style.border_color));
    let exit_style = match beam_phase {
        0 => border.add_modifier(Modifier::DIM),
        1 => border,
        2 => bright_border.add_modifier(Modifier::BOLD),
        _ => border.add_modifier(Modifier::DIM),
    };

    let center_glyph = match center_phase {
        0 => "·",
        1 => "✦",
        2 => "◆",
        3 => "✸",
        4 => "x",
        5 => "X",
        6 => "✶",
        _ => "✦",
    };

    let prism_color_cycle = match tick % 3 {
        0 => Color::White,
        1 => Color::Cyan,
        _ => Color::Magenta,
    };

    let center_style = if center_glyph == "X" {
        Style::default()
            .fg(bright_color(prism_color_cycle))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(prism_color_cycle)
    };

    // Exit beams around the center
    render_glyph(f, x + 0, y + 0, exit_tl, exit_style);
    render_glyph(f, x + 3, y + 1, exit_tl, exit_style);
    render_glyph(f, x + 9, y + 3, exit_br, exit_style);
    render_glyph(f, x + 11, y + 4, exit_br, exit_style);

    // Pulse border corners when beams brighten
    if beam_phase == 2 {
        f.render_widget(Paragraph::new("┏").style(bright_border), Rect::new(area.x, area.y, 1, 1));
        let br_x = area.right() - 1;
        let br_y = area.bottom() - 1;
        f.render_widget(Paragraph::new("┛").style(bright_border), Rect::new(br_x, br_y, 1, 1));
    }

    // Center pulse
    render_glyph(f, x + 6, y + 2, center_glyph, center_style);

    // Entry beams stacked in the top-right corner
    render_glyph(f, x + 11, y + 0, entry_glyph, entry_style);
    render_glyph(f, x + 9, y + 1, entry_glyph, entry_style);
}
