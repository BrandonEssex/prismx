use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    style::{Style, Color},
};
use crate::state::PluginStatus;

pub fn render_plugin_dashboard<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    plugins: &[PluginStatus],
) {
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