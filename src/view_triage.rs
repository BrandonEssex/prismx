use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::text::{Span, Line, Text};
use ratatui::style::{Style, Color};
use ratatui::Frame;

use crate::storage::inbox_storage::InboxState;

pub fn render_triage<B>(f: &mut Frame<B>, area: Rect, inbox: &InboxState) {
    let lines: Vec<Line> = inbox.tasks.iter()
        .map(|task| Line::from(Span::styled(task.clone(), Style::default().fg(Color::White))))
        .collect();

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Inbox Triage").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}