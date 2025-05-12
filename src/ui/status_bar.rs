use crate::state::AppState;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn render_status_bar(state: &AppState) -> Span<'static> {
    if let Some(id) = state.focused_node {
        Span::styled(
            format!("Focused: {} | Sidebar: {}", id, state.sidebar_visible),
            Style::default().fg(Color::Yellow),
        )
    } else {
        Span::styled("No node focused", Style::default().fg(Color::DarkGray))
    }
}