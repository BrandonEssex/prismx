use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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
        .map(|(act, key)| (key.clone(), act.clone()))
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

use crate::ui::components::debug::render_debug_panel;

/// Render the developer diagnostic overlay.
pub fn render_debug_overlay<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState, sticky: bool) {
    render_debug_panel(f, area, state, sticky);
}
