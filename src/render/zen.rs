use chrono::{Datelike, Local};
use ratatui::{
    layout::Alignment,
    prelude::{Backend, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::canvas::prism::render_prism;
use crate::config::theme::ThemeConfig;
use crate::zen::utils::highlight_tags_line;
use crate::state::{AppState, ZenJournalEntry};

/// Extract all #tags from a block of text.
pub fn extract_tags(text: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '#' {
            let mut tag = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                    tag.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            if !tag.is_empty() {
                tags.push(format!("#{}", tag));
            }
        }
    }
    tags
}

/// Render Zen journal history view.
pub fn render_history<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let padding = area.width / 4;
    let usable_width = area.width - padding * 2;
    let breathe = ThemeConfig::load().zen_breathe();

    if state.zen_journal_entries.is_empty() {
        let msg = Paragraph::new("No journal entries yet.").alignment(Alignment::Center);
        let rect = Rect::new(area.x + padding, area.y + area.height / 2, usable_width, 1);
        f.render_widget(msg, rect);
        return;
    }

    let entries = state.filtered_journal_entries();
    let mut blocks: Vec<(u16, Paragraph)> = Vec::new();
    let mut current_label = String::new();

    for (idx, entry) in entries.iter().enumerate().rev() {
        let mut lines: Vec<Line> = Vec::new();

        // Time-grouped label
        if matches!(state.zen_view_mode, crate::state::view::ZenViewMode::Summary) {
            let label = match state.zen_summary_mode {
                crate::state::ZenSummaryMode::Weekly => {
                    format!("Week {}", entry.timestamp.iso_week().week())
                }
                crate::state::ZenSummaryMode::Daily => {
                    let today = Local::now().date_naive();
                    let edate = entry.timestamp.date_naive();
                    if edate == today {
                        "Today".to_string()
                    } else {
                        entry.timestamp.format("%A").to_string()
                    }
                }
            };

            if current_label != label {
                lines.push(Line::from(Span::styled(
                    label.clone(),
                    Style::default().fg(Color::Magenta),
                )));
                current_label = label;
            }
        }

        // Timestamp
        let ts = entry.timestamp.format("%b %d, %Y – %-I:%M%p").to_string();
        lines.push(Line::from(Span::styled(
            ts,
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM),
        )));

        // Tag line
        let tags = extract_tags(&entry.text);
        if !tags.is_empty() {
            lines.push(highlight_tags_line(&tags.join(" ")));
        }

        // Main content
        for l in entry.text.lines() {
            lines.push(highlight_tags_line(l));
        }

        lines.push(Line::from(Span::styled(
            "────────────",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        )));

        if breathe {
            let age = (Local::now() - entry.timestamp).num_milliseconds() as u128;
            for line in lines.iter_mut() {
                crate::ui::animate::fade_line(line, age, 150);
            }
        }

        let mut block = Block::default().borders(Borders::NONE);
        if Some(idx) == state.zen_draft.editing {
            block = block
                .border_style(Style::default().fg(Color::DarkGray))
                .borders(Borders::ALL);
        }

        let h = lines.len() as u16;
        let para = Paragraph::new(lines).block(block);
        blocks.push((h, para));
    }

    let mut y = area.bottom();
    for (h, para) in blocks.into_iter().rev() {
        if y < area.y + h {
            break;
        }
        y -= h;
        let rect = Rect::new(area.x + padding, y, usable_width, h);
        f.render_widget(para, rect);
        if y > area.y {
            y -= 1;
        }
    }

    render_prism(f, area);
}

/// Max block length before splitting
pub const MAX_BLOCK_LEN: usize = 180;

pub fn split_blocks(text: &str) -> Vec<String> {
    text.chars()
        .collect::<Vec<_>>()
        .chunks(MAX_BLOCK_LEN)
        .map(|c| c.iter().collect::<String>())
        .collect()
}
