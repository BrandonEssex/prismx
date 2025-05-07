use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Modifier},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::state::SpotlightState;

pub fn log_event(message: &str) {
    let log_dir = "logs";
    let log_file = format!("{}/spotlight.log", log_dir);

    std::fs::create_dir_all(log_dir).ok();

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}

pub fn render_debug_overlay(f: &mut Frame, state: &SpotlightState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(area);

    let debug_lines = vec![
        Line::from(Span::styled("📊 Spotlight Debug Info", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(format!("Query: {}", state.query)),
        Line::from(format!("Matched Results: {}", state.matched.len())),
        Line::from(format!("Selected Index: {}", state.selected)),
        Line::from(format!("Scope: {:?}", state.scope)),
        Line::from(format!("Debug Enabled: {}", state.debug_enabled)),
    ];

    let debug_paragraph = Paragraph::new(debug_lines)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Spotlight Debug"));

    f.render_widget(debug_paragraph, chunks[1]);
}