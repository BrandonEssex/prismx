// FINAL FULL FILE DELIVERY
// Filename: /src/view_triage.rs

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::storage::inbox_storage::InboxState;

pub fn render_triage(
    f: &mut Frame<'_>,
    area: Rect,
    state: &InboxState,
    context_open: bool,
) {
    let block = Block::default()
        .title("Inbox / Triage")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let mut lines = vec![];
    for (i, task) in state.tasks.iter().enumerate() {
        let prefix = if context_open { format!("#{} > ", i + 1) } else { format!("- ") };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, task),
            Style::default().fg(Color::White),
        )));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No tasks available.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}