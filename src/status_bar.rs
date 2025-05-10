use ratatui::text::Span;
use ratatui::style::{Style, Color};
use crate::state::AppState;

pub fn render_status_bar(state: &AppState) -> Span<'static> {
    let status = if let Some(focused) = state.focused_node {
        format!("Focused: {} | Sidebar: {}", focused, state.sidebar_visible)
    } else {
        "No node focused".to_string()
    };
    Span::styled(status, Style::default().fg(Color::Yellow))
}