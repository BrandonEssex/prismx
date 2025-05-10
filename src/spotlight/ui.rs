use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::text::{Text, Line};
use ratatui::style::{Style, Color};
use ratatui::Frame;

use crate::spotlight::state::SpotlightState;

pub fn render_spotlight_ui<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: ratatui::layout::Rect,
    state: &SpotlightState,
) {
    let input = Paragraph::new(Text::from(Line::from(vec![state.query.clone()])))
        .block(Block::default().title("Spotlight").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(input, area);
}