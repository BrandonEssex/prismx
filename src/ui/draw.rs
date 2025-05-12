// PATCHED: src/ui/draw.rs — Mindmap, Command Bar, and Icon context now visible

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use crate::state::{AppState, View, SidebarView};
use crate::ui::dashboard_widgets::render_dashboard_widget;
use crate::ui::zen_mode::render_zen_mode;
use crate::ui::log_viewer::render_log_viewer;
use crate::ui::mindmap::render_mindmap;
use crate::ui::command_bar::render_command_bar;
use crate::prism_icon::render_prism_icon;
use crate::node_tree::NodeTree;
use crate::ui::sidebar::render_sidebar_panel;

pub fn draw(frame: &mut ratatui::Frame<'_>, app_state: &AppState, tree: &NodeTree) {
    let size = frame.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if matches!(app_state.sidebar, SidebarView::Hidden) { 0 } else { 30 }),
            Constraint::Min(1),
        ])
        .split(layout[0]);

    if app_state.view == View::Zen {
        render_zen_mode(frame, layout[0]);
    }

    if app_state.sidebar != SidebarView::Hidden {
        render_sidebar(frame, chunks[0], &app_state.sidebar);
    }

    match app_state.view {
        View::Dashboard => render_dashboard_widget(frame, chunks[1]),
        View::Log => render_log_viewer(frame, chunks[1]),
        View::Mindmap => render_mindmap(frame, chunks[1], tree),
        View::Export => render_dashboard_widget(frame, chunks[1]), // placeholder
        _ => {}
    }

    if app_state.command_bar_active {
        let w = size.width.min(60);
        let x = (size.width.saturating_sub(w)) / 2;
        let y = size.height / 2;

        let overlay = Rect {
            x,
            y,
            width: w,
            height: 3,
        };

        render_command_bar(frame, overlay, &app_state.command_buffer);
    }

    let icon_area = Rect {
        x: size.width.saturating_sub(10),
        y: 0,
        width: 10,
        height: 1,
    };
    render_prism_icon(frame, icon_area, &app_state.view.to_string());
}

fn render_sidebar(frame: &mut ratatui::Frame<'_>, area: Rect, sidebar: &SidebarView) {
    let help_lines = match sidebar {
        SidebarView::Help => vec![
            "Ctrl+Q: Quit",
            "Ctrl+H: Help",
            "Ctrl+D: Dashboard",
            "Ctrl+Z: Zen",
            "Ctrl+L: Log",
            "Ctrl+M: Mindmap",
            "Ctrl+E: Export",
            "Ctrl+.: Cmd",
        ],
        _ => vec![],
    };

    let title = format!("{:?}", sidebar);
    render_sidebar_panel(frame, area, &title, &help_lines);
}