use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::fs;

pub fn render_plugin_help_overlay<B: Backend>(f: &mut Frame<B>, area: Rect, plugin_id: &str) {
    let help_path = format!("{}/.prismx/plugins/{}/help.md", std::env::var("HOME").unwrap(), plugin_id);
    let content = fs::read_to_string(help_path).unwrap_or_else(|_| "No help found.".to_string());

    let block = Block::default()
        .title(format!("Help: {}", plugin_id))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    f.render_widget(paragraph, layout[0]);
}
