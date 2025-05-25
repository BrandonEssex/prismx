use ratatui::{backend::Backend, layout::Rect, style::{Style, Color}, widgets::{Block, Borders, Paragraph}, Frame};
use crate::state::AppState;

pub fn render_spotlight<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let input = &state.spotlight_input;
    let width = area.width.min(60);
    let x_offset = area.x + (area.width.saturating_sub(width)) / 2;
    let y_offset = area.y + area.height / 3;

    let spotlight_area = Rect::new(x_offset, y_offset, width, 3);

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

    let paragraph = Paragraph::new(format!("> {}", input))
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, spotlight_area);
}
