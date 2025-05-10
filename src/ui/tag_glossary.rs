use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, List, ListItem},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};

use crate::state::AppState;

pub fn render_tag_glossary(f: &mut Frame<'_>, area: Rect, app_state: &AppState) {
    let tags = &app_state.tag_glossary;

    let lines: Vec<Line> = tags
        .iter()
        .map(|tag| {
            Line::from(vec![
                Span::styled(tag.name.clone(), Style::default().fg(Color::Green)),
                Span::raw(" "),
                Span::styled(tag.trust.clone().unwrap_or_default(), Style::default().fg(Color::Yellow)),
            ])
        })
        .collect();

    let block = Block::default().title("Tag Glossary").borders(Borders::ALL);
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}