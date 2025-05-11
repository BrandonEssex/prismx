use ratatui::{
    backend::Backend,
    layout::{Rect, Layout, Direction, Constraint},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    Frame,
};

use crate::plugin::registry::PluginRegistry;

pub fn render_plugin_dashboard(f: &mut Frame<'_>, area: Rect) {
    let plugin_names: Vec<String> = PluginRegistry::list_plugin_names();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); plugin_names.len()])
        .split(area);

    for (i, name) in plugin_names.iter().enumerate() {
        let title = format!("Plugin: {}", name);
        let block = Paragraph::new(Line::from(vec![
            Span::raw("Status: "),
            Span::raw("Active"),
        ]))
        .block(Block::default().title(title).borders(Borders::ALL));
        f.render_widget(block, chunks[i]);
    }
}