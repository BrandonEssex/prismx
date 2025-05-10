use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::Frame;
use crate::state::AppState;

pub fn render_tag_glossary(f: &mut Frame, area: Rect, _state: &AppState) {
    let glossary = Paragraph::new("Tags glossary not yet implemented.")
        .block(Block::default().title("Tags").borders(Borders::ALL));
    f.render_widget(glossary, area);
}