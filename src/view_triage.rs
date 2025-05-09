use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Modifier, Color},
    Frame,
};

use crate::storage::inbox_storage::InboxState;

pub fn render_triage(f: &mut Frame<'_>, area: Rect, state: &InboxState, context_open: bool) {
    let lines: Vec<Line> = state.items.iter().enumerate().map(|(i, item)| {
        let style = if i == state.selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let line = format!("[P{}] {} [{}]", item.priority, item.title, item.shard);
        Line::from(Span::styled(line, style))
    }).collect();

    let block = Block::default()
        .title("Inbox")
        .borders(Borders::ALL);

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);

    if context_open {
        render_context_menu(f, area);
    }
}

fn render_context_menu(f: &mut Frame<'_>, area: Rect) {
    let items = vec!["Mark Done", "Edit", "Archive"];
    let lines: Vec<Line> = items.iter().map(|label| {
        Line::from(Span::styled(*label, Style::default().fg(Color::Magenta)))
    }).collect();

    let block = Block::default()
        .title("Item Menu")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let para = Paragraph::new(lines).block(block);
    let width = 22;
    let height = items.len() as u16 + 2;
    let x = area.width.saturating_sub(width) - 1;
    let y = area.y + 1;

    f.render_widget(para, Rect::new(x, y, width, height));
}