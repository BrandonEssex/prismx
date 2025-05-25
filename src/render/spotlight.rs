use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::state::AppState;
use crate::spotlight::command_preview;

pub fn render_spotlight<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let input = &state.spotlight_input;
    let width = area.width.min(60);
    let x_offset = area.x + (area.width.saturating_sub(width)) / 2;
    let y_offset = area.y + area.height / 3;

    let preview = command_preview(input);
    let height = if preview.is_some() { 4 } else { 3 };
    let spotlight_area = Rect::new(x_offset, y_offset, width, height);

    let arrow = if state.spotlight_just_opened {
        match state.spotlight_animation_frame {
            0 => "→ ",
            1 => "→ → ",
            2 => "→ → → ",
            _ => "",
        }
    } else {
        ""
    };

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
        .title(format!("{}Spotlight", arrow))
        .borders(Borders::ALL)
        .style(Style::default().fg(border_color));

    let mut lines = vec![Line::styled(format!("> {}", input), Style::default().fg(Color::White))];
    if let Some((msg, known)) = preview {
        if known {
            lines.push(Line::from(vec![
                Span::styled("→ ", Style::default().fg(Color::Cyan)),
                Span::styled(msg, Style::default().fg(Color::White)),
            ]));
        } else {
            let style = Style::default().fg(Color::Red).add_modifier(Modifier::DIM);
            lines.push(Line::from(vec![
                Span::styled("⚠ ", style),
                Span::styled(msg, style),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).block(block);

    f.render_widget(paragraph, spotlight_area);
}
