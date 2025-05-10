// FINAL FULL FILE DELIVERY
// Filename: /src/ui/plugin_dashboard.rs

use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    style::{Style, Color},
    Frame,
};

use crate::plugin::status::PluginStatus;

pub fn render_plugin_dashboard(f: &mut Frame<'_>, area: Rect, plugins: &[PluginStatus]) {
    let block = Block::default().title("Plugin Status").borders(Borders::ALL);

    let lines: Vec<Line> = plugins
        .iter()
        .map(|p| {
            let color = match p.status.as_str() {
                "OK" => Color::Green,
                "Warn" => Color::Yellow,
                "Error" => Color::Red,
                _ => Color::Gray,
            };
            Line::from(vec![
                Span::styled(&p.name, Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::styled(&p.status, Style::default().fg(color)),
            ])
        })
        .collect();

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}