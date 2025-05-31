use ratatui::{
    backend::Backend,
    widgets::{Block, Borders, Paragraph, Clear},
    layout::Rect,
    Frame,
};

use unicode_width::UnicodeWidthStr;

use crate::state::AppState;

/// Render a dynamic shortcut overlay showing hotkeys for the active module.
/// Extra hotkeys can be injected by plugins via `plugin::hook`.
pub fn render_dynamic_overlay<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &AppState,
    plugin_keys: &[(String, String)],
) {
    let mut pairs: Vec<(String, String)> = state
        .hotkeys
        .iter()
        .map(|(act, key)| (key.clone(), act.replace('_', " ")))
        .collect();

    pairs.extend_from_slice(plugin_keys);

    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let lines: Vec<String> = pairs
        .iter()
        .map(|(k, v)| format!("{} = {}", k, v))
        .collect();

    let inner_height = area.height.saturating_sub(3);
    let content = Paragraph::new(lines.join("\n"))
        .block(Block::default().title("Shortcuts").borders(Borders::ALL));
    f.render_widget(content, Rect::new(area.x + 1, area.y + 1, area.width - 2, inner_height));
}

use crate::layout::engine::grid_size;

/// Render the developer diagnostic overlay.
///
/// Displays minimal runtime information in a floating panel that can be toggled
/// on demand. The panel shows the active module, current scroll offset, node
/// count and calculated grid size. When BeamX is visible a darker background is
/// used so the overlay remains readable against the animated border.
pub fn render_debug_overlay<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &AppState,
    _sticky: bool,
) {
    use ratatui::style::{Color, Style};
    use ratatui::widgets::{BorderType};

    let (rows, cols) = grid_size(&state.nodes);

    let lines = [
        format!("Module: {}", state.mode),
        format!("Scroll: {},{}", state.scroll_x, state.scroll_y),
        format!("Nodes: {}", state.nodes.len()),
        format!("Grid: {}x{}", rows, cols),
    ];

    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16 + 2;
    let height = lines.len() as u16 + 2;

    // Position the overlay in the top-right with a small padding
    let x = area.right().saturating_sub(width + 1);
    let y = area.y + 1;

    let style = if state.beamx_panel_visible {
        Style::default().fg(Color::White).bg(Color::Rgb(40, 40, 40))
    } else {
        Style::default()
    };

    let rect = Rect::new(x, y, width, height);
    f.render_widget(Clear, rect);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style);
    f.render_widget(block, rect);
    let inner = Rect::new(x + 1, y + 1, width - 2, height - 2);
    f.render_widget(Paragraph::new(lines.join("\n")).style(style), inner);
}

/// Render a tooltip near a node label with its full text.
pub fn render_node_tooltip<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    node_rect: Rect,
    text: &str,
) {
    let mut width = UnicodeWidthStr::width(text) as u16 + 2;
    if width > area.width {
        width = area.width;
    }

    let mut x = node_rect.x;
    if x + width > area.right() {
        x = area.right().saturating_sub(width);
    }

    let mut y = node_rect.y.saturating_sub(1);
    if y + 3 > area.bottom().saturating_sub(1) {
        y = area.bottom().saturating_sub(3);
    }

    let rect = Rect::new(x, y, width, 3);
    f.render_widget(Clear, rect);
    f.render_widget(Block::default().borders(Borders::ALL), rect);

    let mut content = text.to_string();
    let inner_w = width.saturating_sub(2) as usize;
    if UnicodeWidthStr::width(content.as_str()) as u16 > width - 2 {
        content = content.chars().take(inner_w.saturating_sub(1)).collect();
        content.push('…');
    }

    f.render_widget(Paragraph::new(content), Rect::new(x + 1, y + 1, width - 2, 1));
}

/// Render visual layout zones for debugging mindmap placement.
pub fn render_layout_zones<B: Backend>(f: &mut Frame<B>, area: Rect) {
    use ratatui::style::{Color, Style};
    use crate::layout::{GEMX_HEADER_HEIGHT, RESERVED_ZONE_W, RESERVED_ZONE_H};

    let dock_h = 2u16;
    let emblem_rect = Rect::new(
        area.right().saturating_sub(RESERVED_ZONE_W as u16),
        area.y,
        RESERVED_ZONE_W as u16,
        RESERVED_ZONE_H as u16,
    );
    let dock_rect = Rect::new(
        area.x,
        area.bottom().saturating_sub(dock_h),
        area.width,
        dock_h,
    );
    let allowed_rect = Rect::new(
        area.x,
        area.y + GEMX_HEADER_HEIGHT as u16,
        area.width.saturating_sub(RESERVED_ZONE_W as u16),
        area.height.saturating_sub(GEMX_HEADER_HEIGHT as u16 + dock_h),
    );
    let gray = Style::default().bg(Color::DarkGray);
    let red = Style::default().bg(Color::Red);
    f.render_widget(Block::default().style(red), emblem_rect);
    f.render_widget(Block::default().style(gray), dock_rect);
    let zone = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    f.render_widget(zone, allowed_rect);
}
