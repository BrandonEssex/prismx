use ratatui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::ui::layout::Rect;

use crate::state::AppState;
use crate::plugin::registry::{self, PluginManifest};
use crate::plugin::panel::render_plugin_panel;

/// Primary plugin module renderer.
/// Renders plugin panel (list view) and optional sync badge.
pub fn render_plugin<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    // Update plugin registry state
    registry::tick();

    // Render the plugin panel (cards/list from plugin::panel.rs)
    render_plugin_panel(f, area, state);

    // Optional [sync] badge when plugin registry changes
    if registry::sync_badge() {
        let label = Paragraph::new("[sync]").style(Style::default().fg(Color::Green));
        let width = 6u16;
        f.render_widget(label, Rect::new(area.right().saturating_sub(width + 1), area.y, width, 1));
    }
}
