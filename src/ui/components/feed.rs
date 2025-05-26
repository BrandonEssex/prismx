use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use crate::state::ZenJournalEntry;

pub fn render_feed<B: Backend>(f: &mut Frame<B>, area: Rect, entries: &[ZenJournalEntry]) {
    let mut lines: Vec<Line> = Vec::new();
    for entry in entries.iter().rev() {
        lines.push(Line::from(Span::styled(
            entry.timestamp.format("%Y-%m-%d %H:%M").to_string(),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(entry.text.clone()));
        lines.push(Line::from(Span::styled("\u{2500}".repeat(area.width as usize), Style::default().fg(Color::DarkGray))));
    }
    let para = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
    f.render_widget(para, area);
}
