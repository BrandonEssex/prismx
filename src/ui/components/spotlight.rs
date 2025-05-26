use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};

use crate::state::AppState;
use crate::spotlight::{command_preview, command_suggestions};
use crate::theme;

pub fn render_spotlight<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let input = &state.spotlight_input;
    let base_width = area.width.min(60);
    let scale = if state.spotlight_just_opened {
        match state.spotlight_animation_frame {
            0 => 0.90,
            1 => 0.97,
            _ => 1.0,
        }
    } else {
        1.0
    };
    let width = ((base_width as f32) * scale) as u16;
    let width = width.max(3).min(base_width);
    let x_offset = area.x + (area.width.saturating_sub(width)) / 2;
    let y_offset = area.y + area.height / 3;

    let preview = command_preview(input);
    let matches = command_suggestions(input);
    let mut height = if preview.is_some() { 4 } else { 3 };
    // Ensure Spotlight stays above the status bar
    height = height.min(area.height.saturating_sub(1));
    let suggestion_count = matches.len().min(5) as u16;
    let total_height = height + suggestion_count;
    let spotlight_area = Rect::new(x_offset, y_offset, width, height);

    let border_color = if state.spotlight_just_opened {
        match state.spotlight_animation_frame {
            0 => Color::LightBlue,
            1 => Color::Cyan,
            _ => Color::White,
        }
    } else {
        Color::Cyan
    };

    let block = Block::default()
        .title("Spotlight")
        .borders(Borders::ALL)
        .style(Style::default().fg(border_color).bg(Color::Black));

    let spot_style = theme::get_style("spotlight").fg(Color::White);
    let mut lines = vec![
        Line::styled(
            format!("> {}", input),
            spot_style
                .add_modifier(Modifier::BOLD),
        ),
    ];

    if let Some((msg, known)) = preview {
        if known {
            lines.push(Line::from(vec![
                Span::styled(
                    "→ ",
                    Style::default().fg(Color::Cyan).bg(Color::Black),
                ),
                Span::styled(
                    msg,
                    Style::default().fg(Color::White).bg(Color::Black),
                ),
            ]));
        } else {
            let style = Style::default().fg(Color::Red).bg(Color::Black);
            lines.push(Line::from(vec![
                Span::styled("⚠ ", style),
                Span::styled(msg, style),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).block(block);

    // Clear background area and render solid black base layer
    let bg_rect = Rect::new(x_offset, y_offset, width, total_height);
    f.render_widget(Clear, bg_rect);
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        bg_rect,
    );

    f.render_widget(paragraph, spotlight_area);

    for (i, suggestion) in matches.iter().take(5).enumerate() {
        let y = y_offset + 2 + i as u16;
        if y >= area.y + area.height.saturating_sub(1) {
            break;
        }
        let mut style = spot_style;
        if Some(i) == state.spotlight_suggestion_index {
            style = style.fg(Color::Black).bg(Color::White);
        }
        f.render_widget(
            Paragraph::new(*suggestion).style(style),
            Rect::new(x_offset, y, width, 1),
        );
    }
}
