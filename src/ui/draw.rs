// src/ui/draw.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use crate::state::{AppState, View, SidebarView};
use crate::ui::help_overlay::render_help_overlay;
use crate::ui::dashboard_widgets::render_dashboard_widget;
use crate::ui::zen_mode::render_zen_mode;
use crate::ui::log_viewer::render_log_viewer;
use crate::ui::plugin_dashboard::render_plugin_dashboard;
use crate::ui::command_bar::render_command_bar;
use crate::ui::mindmap::render_mindmap;
use crate::prism_icon::render_prism_icon;
use crate::plugin::registry::PluginRegistry;

pub fn draw(frame: &mut ratatui::Frame<'_>, app_state: &AppState) {
    let size = frame.size();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if matches!(app_state.sidebar, SidebarView::Hidden) { 0 } else { 30 }),
            Constraint::Min(1),
            Constraint::Length(10),
        ])
        .split(main_layout[0]);

    // Sidebar
    if app_state.sidebar != SidebarView::Hidden {
        render_sidebar(frame, chunks[0], &app_state.sidebar);
    }

    // Main content
    match app_state.view {
        View::Dashboard => {
            render_dashboard_widget(frame, chunks[1]);
            render_plugin_dashboard(frame, chunks[2], &app_state.plugin_registry);
        }
        View::Zen => render_zen_mode(frame, chunks[1]),
        View::Log => render_log_viewer(frame, chunks[1]),
        View::Mindmap => render_mindmap(frame, chunks[1]),
        View::Export => render_dashboard_widget(frame, chunks[1]),
    }

    // Command bar (bottom)
    render_command_bar(frame, main_layout[1], "");
    render_prism_icon(frame, chunks[2], &format!("{:?}", app_state.view));
}

fn render_sidebar(frame: &mut ratatui::Frame<'_>, area: Rect, sidebar: &SidebarView) {
    match sidebar {
        SidebarView::Help => render_help_overlay(frame, area, *sidebar),
        _ => {}
    }
}