// src/spotlight/ui.rs

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::spotlight::state::SpotlightState;
use crate::spotlight::debug::render_debug_overlay;

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

    let input = Paragraph::new(Line::from(vec![Span::raw(state.query.as_str())]))
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Spotlight Search"));
    f.render_widget(input, chunks[0]);

    let items: Vec<ListItem> = state
        .matched
        .iter()
        .enumerate()
        .map(|(i, res)| {
            let mut style = Style::default();
            if i == state.selected {
                style = style.add_modifier(Modifier::REVERSED);
            }
            ListItem::new(Line::from(vec![Span::styled(res.display_title.clone(), style)]))
        })
        .collect();

    let results = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(results, chunks[1]);

    let footer = Paragraph::new("↑↓ to navigate • Enter to open • Esc to exit • Ctrl+D: debug")
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);

    if state.debug_enabled {
        render_debug_overlay(f, state, size);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
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
        .split(popup_layout[1])[1]
}