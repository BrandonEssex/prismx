use crate::spotlight::state::SpotlightState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_overlay(frame: &mut Frame, state: &mut SpotlightState) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(size);

    let input = Paragraph::new(state.query.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Spotlight Search"))
        .alignment(Alignment::Left);
    frame.render_widget(input, chunks[0]);

    let results: Vec<ListItem> = state
        .matched
        .iter()
        .map(|res| {
            let line = Line::from(vec![Span::styled(res.title.clone(), Style::default())]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(results)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    frame.render_widget(list, chunks[1]);
}