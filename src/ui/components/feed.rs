use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use crate::state::ZenJournalEntry;
use crate::zen::journal::extract_tags;

pub fn render_feed<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    entries: &[ZenJournalEntry],
    tag_filter: Option<&str>,
    summary: bool,
) {
    let mut blocks: Vec<Vec<Line>> = Vec::new();
    let mut current_date = String::new();
    for entry in entries {
        if let Some(tag) = tag_filter {
            if !extract_tags(&entry.text).iter().any(|t| t.eq_ignore_ascii_case(tag)) {
                continue;
            }
        }

        if summary {
            let d = entry.timestamp.format("%Y-%m-%d").to_string();
            if current_date != d {
                blocks.push(vec![Line::from(Span::styled(d.clone(), Style::default().fg(Color::Magenta)))]);
                current_date = d;
            }
        }

        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(
            entry.timestamp.format("%Y-%m-%d %H:%M").to_string(),
            Style::default().fg(Color::DarkGray),
        )));
        for line in entry.text.lines() {
            let mut spans = Vec::new();
            for token in line.split_whitespace() {
                if token.starts_with('#') {
                    spans.push(Span::styled(token, Style::default().fg(Color::Blue)));
                } else {
                    spans.push(Span::raw(token));
                }
                spans.push(Span::raw(" "));
            }
            lines.push(Line::from(spans));
        }
        lines.push(Line::from(Span::styled("\u{2500}".repeat(area.width as usize), Style::default().fg(Color::DarkGray))));
        blocks.push(lines);
    }

    let mut y = area.bottom();
    for block in blocks.into_iter().rev() {
        let h = block.len() as u16;
        if y < area.y + h { break; }
        y -= h;
        let rect = Rect::new(area.x, y, area.width, h);
        let para = Paragraph::new(block).block(Block::default().borders(Borders::NONE));
        f.render_widget(para, rect);
    }
}
