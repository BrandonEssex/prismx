// src/ui/draw.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use crate::state::{AppState, View, SidebarView};
use crate::ui::help_overlay::render_help_overlay;
use crate::ui::dashboard_widgets::render_dashboard_widget;
use crate::ui::zen_mode::render_zen_mode;
use crate::ui::log_viewer::render_log_viewer;

pub fn draw(frame: &mut ratatui::Frame<'_>, app_state: &AppState) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if matches!(app_state.sidebar, SidebarView::Hidden) { 0 } else { 30 }),
            Constraint::Min(1),
        ])
        .split(size);

    if app_state.sidebar != SidebarView::Hidden {
        render_sidebar(frame, chunks[0], &app_state.sidebar);
    }

    match app_state.view {
        View::Dashboard => render_dashboard_widget(frame, chunks[1]),
        View::Zen => render_zen_mode(frame, chunks[1]),
        View::Log => render_log_viewer(frame, chunks[1]),
        _ => {}
    }
}

fn render_sidebar(frame: &mut ratatui::Frame<'_>, area: Rect, sidebar: &SidebarView) {
    match sidebar {
        SidebarView::Help => render_help_overlay(frame, area, *sidebar),
        _ => {}
    }
}