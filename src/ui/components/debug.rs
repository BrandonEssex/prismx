use ratatui::{prelude::*, widgets::Paragraph};

use crate::state::AppState;

/// Render debug information when [`AppState::debug_input_mode`] is enabled.
pub fn render_debug<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    if !state.debug_input_mode {
        return;
    }
    let msg = if state.status_message.is_empty() {
        "Debug Mode".to_string()
    } else {
        state.status_message.clone()
    };
    let para = Paragraph::new(msg).style(Style::default().fg(Color::Yellow));
    f.render_widget(para, area);
}
