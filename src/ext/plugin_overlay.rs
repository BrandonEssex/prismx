// src/ext/plugin_overlay.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_plugin_overlay(frame: &mut Frame<'_>, area: Rect, plugin_id: &str, alert_count: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Plugin Overlay");

    let color = if alert_count > 0 {
        Color::Red
    } else {
        Color::Green
    };

    let content = vec![
        Line::from(Span::styled(
            format!("Plugin ID: {}", plugin_id),
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            format!("Alerts: {}", alert_count),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )),
    ];

    let paragraph = Paragraph::new(content).block(block);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}