use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use chrono::{Datelike, Local};
use crate::config::theme::ThemeConfig;
use crate::state::AppState;
use crate::state::view::ZenLayoutMode;
use crate::state::ZenViewMode;
use crate::zen::utils::{highlight_tags_line, extract_tags, parse_tags};
use crate::beamx::render_full_border;

/// Public render entry point for Journal view
pub fn render_zen_journal<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    render_history(f, area, state);
    render_full_border(f, area, &state.beam_style_for_mode(&state.mode), true, false);
}

/// Shared logic for rendering all journal entries
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

        if matches!(state.zen_layout_mode, ZenLayoutMode::Summary) {
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

        let ts = entry.timestamp.format("%b %d, %Y – %-I:%M%p").to_string();
        lines.push(Line::from(Span::styled(
            ts,
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM),
        )));

        if !entry.tags.is_empty() {
            lines.push(highlight_tags_line(&entry.tags.join(" ")));
        }

        for l in entry.text.lines() {
            lines.push(highlight_tags_line(l));
        }

        lines.push(Line::from(Span::styled(
            "─".repeat(12),
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        )));

        if breathe {
            let age = Local::now()
                .signed_duration_since(entry.timestamp)
                .num_milliseconds() as u128;
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

        let para = Paragraph::new(lines).block(block);
        let h = 5; // estimated height
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
}
