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

use crate::ui::debug::render_debug_panel;

/// Render the developer diagnostic overlay.
pub fn render_debug_overlay<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, sticky: bool) {
    render_debug_panel(f, area, state, sticky);
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
