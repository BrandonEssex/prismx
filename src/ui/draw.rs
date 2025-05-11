use crate::state::AppState;
use crate::ui::status_icon::render_status_icon;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn draw_ui(f: &mut Frame<'_>, state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    let body_area = chunks[0];
    let icon_area = chunks[1];

    // Example render call
    render_status_icon(f, icon_area);
}