use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::ZenJournalEntry;
use crate::ui::animate::fade_line;
use crate::config::theme::ThemeConfig;
use crate::ui::components::feed::extract_tags;
use crate::zen::utils::extract_tags;


pub fn render_feed<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    entries: &[ZenJournalEntry],
    tag_filter: Option<&str>,
    summary: bool,
    weekly: bool,
) {
    let breathe = ThemeConfig::load().zen_breathe();
    let mut blocks: Vec<Vec<Line>> = Vec::new();
    let mut current_label = String::new();

    for (idx, entry) in entries.iter().enumerate().rev() {
        // Filter by tag if present
        if let Some(tag) = tag_filter {
            if !extract_tags(&entry.text)
                .iter()
                .any(|t| t.eq_ignore_ascii_case(tag))
            {
                continue;
            }
        }

        // Summary grouping headers
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

        // Entry content
        let mut lines: Vec<Line> = Vec::new();
        let mut ts = entry.timestamp.format("%Y-%m-%d %H:%M").to_string();
        if entry.prev_text.is_some() {
            ts.push_str(" (edited)");
        }

        lines.push(Line::from(Span::styled(
            ts,
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM),
        )));

        for line in entry.text.lines() {
            let mut spans = Vec::new();
            for token in line.split_whitespace() {
                if token.starts_with('#') {
                    if token.eq_ignore_ascii_case("#DONE") {
                        spans.push(Span::styled(
                            token,
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ));
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

        lines.push(Line::from(Span::styled(
            "\u{2500}".repeat(area.width as usize),
            Style::default().fg(Color::DarkGray),
        )));

        // Fade-in animation
        if breathe {
            let age = (chrono::Local::now() - entry.timestamp).num_milliseconds() as u128;
            for line in lines.iter_mut() {
                fade_line(line, age, 150);
            }
        }

        blocks.push(lines);
    }

    // Layout from bottom up
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
}
