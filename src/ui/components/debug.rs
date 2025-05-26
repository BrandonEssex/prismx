use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{plugin::registry::registry, state::AppState, layout::subtree_depth};

fn last_log_line() -> Option<String> {
    use std::fs;
    let entries = fs::read_dir("logs").ok()?;
    let mut files: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    files.sort_by_key(|e| e.file_name());
    let path = files.last()?.path();
    let content = fs::read_to_string(path).ok()?;
    content.lines().rev().find(|l| !l.trim().is_empty()).map(|s| s.to_string())
}

pub fn render_debug_panel<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, sticky: bool) {
    let depth = state
        .root_nodes
        .iter()
        .map(|id| subtree_depth(&state.nodes, *id) as usize)
        .max()
        .unwrap_or(0);

    let selected = if state.mode == "gemx" {
        match state.selected {
            Some(id) => format!("Selected: {}", id),
            None => "Selected: -".to_string(),
        }
    } else if state.mode == "zen" {
        format!("File: {}", state.zen_current_filename)
    } else {
        String::new()
    };

    let mut lines = vec![
        format!("Module: {}", state.mode),
        format!("Zoom: {:.1}x", state.zoom_scale),
        format!("Scroll: {},{}", state.scroll_x, state.scroll_y),
        selected,
        format!("Auto-arrange: {}", if state.auto_arrange { "ON" } else { "OFF" }),
        format!("Nodes: {} Depth: {}", state.nodes.len(), depth),
        format!("Plugins: {} / {}", state.plugin_host.active.len(), registry().len()),
    ];

    if let Some(log) = last_log_line() {
        lines.push(format!("Log: {}", log));
    }

    let block = Block::default()
        .title(if sticky { "Debug*" } else { "Debug" })
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let inner_height = area.height.saturating_sub(2);
    f.render_widget(block, area);
    f.render_widget(
        Paragraph::new(lines.join("\n")).style(Style::default()),
        Rect::new(area.x + 1, area.y + 1, area.width - 2, inner_height),
    );
}
