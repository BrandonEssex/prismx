use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::spotlight::state::SpotlightState;

pub fn render_overlay(frame: &mut Frame, state: &mut SpotlightState) {
    let size = frame.size();
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(size);

    let input = Paragraph::new(state.query.as_str())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Spotlight"));
    frame.render_widget(input, area[0]);

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
            ListItem::new(Line::from(vec![Span::styled(&res.display_title, style)]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(list, area[1]);

    let footer = Paragraph::new("↑↓ to navigate • Enter to open • Esc to exit • Ctrl+D debug")
        .alignment(Alignment::Center);
    frame.render_widget(footer, area[2]);
}