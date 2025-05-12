// src/ui/plugin_dashboard.rs

use ratatui::layout::Rect;
use ratatui::text::{Span, Line};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use crate::plugin::registry::PluginRegistry;

pub fn render_plugin_dashboard(frame: &mut Frame<'_>, area: Rect, registry: &PluginRegistry) {
    let slots = registry.all();
    let lines: Vec<Line> = slots.iter()
        .map(|name| Line::from(vec![Span::raw(format!("Plugin: {}", name))]))
        .collect();

    let block = Block::default().borders(Borders::ALL).title("Plugins");
    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}
