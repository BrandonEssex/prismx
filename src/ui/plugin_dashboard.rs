use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    Frame,
};

use crate::plugin::registry::PluginRegistry;

pub fn render_plugin_dashboard(f: &mut Frame<'_>, area: Rect) {
    let plugin_names = PluginRegistry::list_plugin_names();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3); plugin_names.len()])
        .split(area);

    for (i, name) in plugin_names.iter().enumerate() {
        let line = Line::from(vec![
            Span::raw("Status: "),
            Span::raw("Active"),
        ]);
        let block = Paragraph::new(line)
            .block(Block::default().title(format!("Plugin: {}", name)).borders(Borders::ALL));
        f.render_widget(block, layout[i]);
    }
}