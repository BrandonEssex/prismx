use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use std::fs;

pub fn render_audit_panel<B: Backend>(f: &mut Frame<B>, area: Rect, log_path: &str) {
    let content = fs::read_to_string(log_path).unwrap_or_else(|_| "No logs found.".to_string());

    let block = Block::default()
        .title("Audit Log")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightBlue));

    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default().fg(Color::White));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    f.render_widget(paragraph, layout[0]);
}
