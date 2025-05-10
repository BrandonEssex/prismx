use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Text, Span, Line};
use ratatui::style::{Style, Color};
use ratatui::Frame;

use crate::spotlight::state::SpotlightState;

pub fn render_spotlight_ui(f: &mut Frame<'_>, area: Rect, state: &SpotlightState) {
    let input = Paragraph::new(Text::from(Line::from(vec![
        Span::raw(state.query.clone())
    ])))
    .block(Block::default().title("Spotlight").borders(Borders::ALL))
    .style(Style::default().fg(Color::White));

    f.render_widget(input, area);
}