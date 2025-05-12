// src/ui/draw.rs

use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use crate::state::{AppState, View, SidebarView};
use crate::ui::help_overlay::render_help_overlay;
use crate::ui::dashboard_widgets::render_dashboard_widget;
use crate::ui::zen_mode::render_zen_mode;
use crate::ui::log_viewer::render_log_viewer;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app_state: &AppState) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if matches!(app_state.sidebar, SidebarView::Hidden) { 0 } else { 30 }),
            Constraint::Min(1),
        ].as_ref())
        .split(size);

    // Render sidebar if visible
    if app_state.sidebar != SidebarView::Hidden {
        render_sidebar(frame, chunks[0], &app_state.sidebar);
    }

    // Render main content based on View
    match app_state.view {
        View::Dashboard => render_dashboard_widget(frame, chunks[1]),
        View::Zen => render_zen_mode(frame, chunks[1]),
        View::Log => render_log_viewer(frame, chunks[1]),
        _ => {} // Future: Mindmap, Export, etc.
    }
}

fn render_sidebar<B: Backend>(frame: &mut Frame<B>, area: Rect, sidebar: &SidebarView) {
    match sidebar {
        SidebarView::Help => render_help_overlay(frame, area, *sidebar),
        _ => {
            // Placeholder for other sidebars
        }
    }
}