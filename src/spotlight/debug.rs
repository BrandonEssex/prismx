// src/spotlight/debug.rs

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::spotlight::state::SpotlightState;

pub fn render_debug_overlay(f: &mut Frame, state: &SpotlightState) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(size);

    let debug_text = vec![
        Line::from(vec![Span::styled(
            "📊 Spotlight Debug Info",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw(format!("Query: {}", state.query))]),
        Line::from(vec![Span::raw(format!("Matched Results: {}", state.matched.len()))]),
        Line::from(vec![Span::raw(format!("Selected Index: {}", state.selected))]),
        Line::from(vec![Span::raw(format!("Debug Enabled: {}", state.debug_enabled))]),
    ];

    let debug_paragraph = Paragraph::new(debug_text)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Spotlight Debug"));

    f.render_widget(debug_paragraph, chunks[1]);
}