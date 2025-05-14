// Author: Brandon Essex
// TUI draw function for PrismX / GemX

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::state::{AppState, View};
use crate::ui::{
    render_sidebar,
    render_mindmap,
    render_dashboard,
    render_plugin_overlay,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(1)].as_ref())
        .split(f.size());

    // Render Sidebar
    render_sidebar(f, app, chunks[0]);

    // Render Main View
    match app.current_view {
        View::Mindmap => render_mindmap(f, app, chunks[1]),
        View::Dashboard => render_dashboard(f, app, chunks[1]),
        View::Plugin => render_plugin_overlay(f, app, chunks[1]),
    }
}
