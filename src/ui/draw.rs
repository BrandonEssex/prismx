// src/ui/draw.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use crate::state::{AppState, View, SidebarView};
use crate::ui::help_overlay::render_help_overlay;
use crate::ui::dashboard_widgets::render_dashboard_widget;
use crate::ui::zen_mode::render_zen_mode;
use crate::ui::log_viewer::render_log_viewer;
use crate::ui::mindmap::render_mindmap;
use crate::ui::command_bar::render_command_bar;
use crate::prism_icon::render_prism_icon;
use crate::node_tree::NodeTree;

pub fn draw(frame: &mut ratatui::Frame<'_>, app_state: &AppState, tree: &NodeTree) {
    let size = frame.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),            // Main content
            Constraint::Length(3),         // Command bar
        ])
        .split(size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if matches!(app_state.sidebar, SidebarView::Hidden) { 0 } else { 30 }),
            Constraint::Min(1),
        ])
        .split(layout[0]);

    if app_state.sidebar != SidebarView::Hidden {
        render_sidebar(frame, chunks[0], &app_state.sidebar);
    }

    match app_state.view {
        View::Dashboard => render_dashboard_widget(frame, chunks[1]),
        View::Zen => render_zen_mode(frame, chunks[1]),
        View::Log => render_log_viewer(frame, chunks[1]),
        View::Mindmap => render_mindmap(frame, chunks[1], tree),
        _ => {}
    }

    render_command_bar(frame, layout[1], "");

    // PrismX icon in top-right corner
    let icon_area = Rect {
        x: size.width.saturating_sub(8),
        y: 0,
        width: 7,
        height: 1,
    };
    render_prism_icon(frame, icon_area, "default");
}

fn render_sidebar(frame: &mut ratatui::Frame<'_>, area: Rect, sidebar: &SidebarView) {
    match sidebar {
        SidebarView::Help => render_help_overlay(frame, area, *sidebar),
        _ => {}
    }
}