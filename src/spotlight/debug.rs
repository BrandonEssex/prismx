use crate::spotlight::state::SpotlightState;
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color};

pub fn debug_spotlight_state(state: &SpotlightState) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Query: ", Style::default().fg(Color::Yellow)),
            Span::raw(&state.query),
        ]),
        Line::from(vec![
            Span::styled("Matches: ", Style::default().fg(Color::Green)),
            Span::raw(state.matched.len().to_string()),
        ]),
    ]
}