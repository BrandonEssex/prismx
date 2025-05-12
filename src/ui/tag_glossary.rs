use crate::state::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_tag_glossary(f: &mut Frame<'_>, area: Rect, _state: &AppState) {
    let glossary = Paragraph::new("Tags glossary not yet implemented.")
        .block(Block::default().title("Tags").borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(glossary, area);
}