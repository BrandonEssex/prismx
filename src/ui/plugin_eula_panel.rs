use std::fs;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_eula_panel<B: Backend>(f: &mut Frame<B>, area: Rect, plugin_id: &str) {
    let path = format!("{}/.prismx/plugins/{}/eula.md", std::env::var("HOME").unwrap(), plugin_id);
    let text = fs::read_to_string(&path).unwrap_or_else(|_| "No EULA available.".to_string());

    let block = Block::default()
        .title(format!("Plugin: {} — EULA", plugin_id))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));

    let layout = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(area);

    f.render_widget(paragraph, layout[0]);
}
