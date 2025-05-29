use ratatui::{backend::Backend, layout::Rect, style::Style, widgets::{Block, Borders, Paragraph}, Frame};
use crate::state::AppState;
use crate::ui::status::status_line;

pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    use std::time::Duration;

    if let Some(last) = state.status_message_last_updated {
        if last.elapsed() > Duration::from_secs(4) {
            state.status_message.clear();
            state.status_message_last_updated = None;
        }
    }

    let default_status = status_line(state);
    let display_string = if state.status_message.is_empty() {
        default_status
    } else {
        state.status_message.clone()
    };

    let block = Block::default().borders(Borders::ALL).title("Status");
    let content = Paragraph::new(display_string).style(Style::default());
    f.render_widget(block, area);
    let inner_width = area.width.saturating_sub(2);
    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, inner_width, 1));
}
