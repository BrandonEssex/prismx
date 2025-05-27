use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Style, Modifier};
use ratatui::text::{Line, Span};
use crate::state::ZenJournalEntry;
use crate::zen::journal::extract_tags;
use chrono::Datelike;

pub fn render_feed<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    entries: &[ZenJournalEntry],
    tag_filter: Option<&str>,
    summary: bool,
    weekly: bool,
) {
    let mut blocks: Vec<Vec<Line>> = Vec::new();
    let mut current_label = String::new();

    for entry in entries.iter().rev() {
        // Apply tag filter if present
        if let Some(tag) = tag_filter {
            if !extract_tags(&entry.text)
                .iter()
                .any(|t| t.eq_ignore_ascii_case(tag))
            {
                continue;
            }
        }

        // Summary grouping
        if summary {
            let label = if weekly {
                format!("Week {}", entry.timestamp.iso_week().week())
            } else {
                let today = chrono::Local::now().date_naive();
                let edate = entry.timestamp.date_naive();
                if edate == today {
                    "Today".to_string()
                } else {
                    entry.timestamp.format("%A").to_string()
                }
            };

            if current_label != label {
                blocks.push(vec![Line::from(Span::styled(
                    label.clone(),
                    Style::default().fg(Color::Magenta),
                ))]);
                current_label = label;
            }
        }

        let mut lines: Vec<Line> = Vec::new();
        let mut ts = entry.timestamp.format("%Y-%m-%d %H:%M").to_string();
        if entry.prev_text.is_some() {
            ts.push_str(" (edited)");
        }

        lines.push(Line::from(Span::styled(
            ts,
            Style::default().fg(Color::DarkGray),
        )));

        for line in entry.text.lines() {
            lines.push(Line::from(line));
        }

        lines.push(Line::from(Span::styled(
            "────────────",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        )));

        blocks.push(lines);
    }

    let mut y = area.bottom();
    for block in blocks.into_iter().rev() {
        let h = block.len() as u16;
        if y < area.y + h {
            break;
        }
        y -= h;
        let rect = Rect::new(area.x + 1, y, area.width - 2, h);
        let para = Paragraph::new(block).block(Block::default().borders(Borders::NONE));
        f.render_widget(para, rect);
        if y > area.y {
            y -= 1;
        }
    }


        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(
            ts,
            Style::default().fg(Color::DarkGray),
        )));
        for line in entry.text.lines() {
            let mut spans = Vec::new();
            for token in line.split_whitespace() {
                if token.starts_with('#') {
                    if token.eq_ignore_ascii_case("#DONE") {
                        spans.push(
                            Span::styled(
                                token,
                                Style::default()
                                    .fg(Color::DarkGray)
                                    .add_modifier(Modifier::DIM),
                            ),
                        );
                    } else {
                        spans.push(Span::styled(token, Style::default().fg(Color::Blue)));
                    }
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
