use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::spotlight::state::SpotlightState;

pub fn render_debug_overlay(f: &mut Frame, state: &SpotlightState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(6)])
        .split(area);

    let debug_lines = vec![
        Line::from(vec![Span::styled(
            "Debug Overlay",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw(format!("Query: {}", state.query))]),
        Line::from(vec![Span::raw(format!("Matched Results: {}", state.matched.len()))]),
        Line::from(vec![Span::raw(format!("Selected Index: {}", state.selected))]),
        Line::from(vec![Span::raw(format!("Debug Enabled: {}", state.debug_enabled))]),
    ];

    let debug_block = Paragraph::new(debug_lines)
        .block(Block::default().borders(Borders::ALL).title("Spotlight Debug"))
        .alignment(Alignment::Left);

    f.render_widget(debug_block, chunks[1]);
}