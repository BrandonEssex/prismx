use ratatui::{
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

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(1)].as_ref())
        .split(f.size());

    render_sidebar(f, app, chunks[0]);

    match app.current_view {
        View::Mindmap => render_mindmap(f, app, chunks[1]),
        View::Dashboard => render_dashboard(f, app, chunks[1]),
        View::Plugin => render_plugin_overlay(f, app, chunks[1]),
    }
}

// src/ui/sidebar.rs
use ratatui::{
    layout::Rect,
    style::{Style, Modifier},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::state::AppState;

pub fn render_sidebar(f: &mut Frame, _app: &AppState, area: Rect) {
    let items = vec![
        ListItem::new("Mindmap"),
        ListItem::new("Dashboard"),
        ListItem::new("Plugins"),
    ];

    let list = List::new(items)
        .block(Block::default().title("Views").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}
