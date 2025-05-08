use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::state::SpotlightState;

pub fn render_overlay(f: &mut Frame, state: &mut SpotlightState) {
    let size = f.size();
    let area = centered_rect(80, 60, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(area);

    let input = Paragraph::new(Line::from(state.query.clone()))
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Spotlight Search"));
    f.render_widget(input, chunks[0]);

    let items: Vec<ListItem> = state
        .matched
        .iter()
        .enumerate()
        .map(|(i, res)| {
            let style = if i == state.selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![Span::styled(res.title.clone(), style)]))
        })
        .collect();

    let results = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_symbol(">> ");
    f.render_widget(results, chunks[1]);

    let footer = Paragraph::new("↑↓ to navigate • Enter to open • Esc to exit • Ctrl+D: debug")
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}