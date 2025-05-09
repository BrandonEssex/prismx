use ratatui::{
    Frame,
    layout::{Rect},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    style::{Style, Color, Modifier}
};
use crate::state::AppState;

pub fn render_tag_glossary<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    app_state: &AppState,
) {
    let block = Block::default().title("Tag Glossary").borders(Borders::ALL);
    let tags = &app_state.tag_glossary;

    let items: Vec<ListItem> = tags
        .iter()
        .map(|tag| {
            let meta = format!(
                "[{}] ({})",
                tag.role,
                tag.source
            );
            let line = Line::from(vec![
                Span::styled(tag.name.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled(meta, Style::default().fg(Color::Gray)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::ITALIC));

    f.render_widget(list, area);
}