use crate::state::{AppState, DebugSnapshot};
use std::time::Instant;
use crate::plugin::registry::registry_filtered;
use crate::state::PluginTagFilter;


use ratatui::{
    backend::Backend,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::ui::layout::Rect;

use crate::layout::subtree_depth;


/// Render debug information when [`AppState::debug_input_mode`] is enabled.
pub fn render_debug<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let plugins = registry_filtered(PluginTagFilter::All);
    if !state.debug_input_mode {
        return;
    }
    let msg = if state.status_message.is_empty() {
        "Debug Mode".to_string()
    } else {
        state.status_message.clone()
    };
    let para = Paragraph::new(msg).style(Style::default().fg(Color::Yellow));
    f.render_widget(para, area);
}

/// Capture a debug snapshot of the current app state and write it to the
/// `logs/snapshots/` directory. The filename is timestamped using local time.
pub fn write_debug_snapshot(state: &mut AppState) {
    let snapshot = DebugSnapshot::from_state(state);
    match crate::logger::write_snapshot(&snapshot) {
        Ok(path) => {
            state.status_message = format!("Snapshot saved to {}", path.display());
            state.status_message_last_updated = Some(Instant::now());
        }
        Err(err) => {
            state.status_message = format!("Snapshot failed: {err}");
            state.status_message_last_updated = Some(Instant::now());
        }
    }
}

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
        format!("Plugins: {} / {}", state.plugin_host.active.len(), registry_filtered(PluginTagFilter::All)
.len()),
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

