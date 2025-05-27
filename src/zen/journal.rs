use chrono::Local;
use crate::state::{AppState, ZenJournalEntry};
use crate::canvas::prism::render_prism;
use chrono::Datelike;


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

use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::prelude::Backend;
use ratatui::prelude::Rect;
use crate::config::theme::ThemeConfig;
use crate::zen::utils::highlight_tags_line;

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

        if matches!(state.zen_view_mode, crate::state::view::ZenViewMode::Summary) {
            let label = match state.zen_summary_mode {
                crate::state::ZenSummaryMode::Weekly => {
                    format!("Week {}", entry.timestamp.iso_week().week())
                }
                crate::state::ZenSummaryMode::Daily => {
                    let today = chrono::Local::now().date_naive();
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
                    label,
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

        let tags = extract_tags(&entry.text);
        if !tags.is_empty() {
            lines.push(highlight_tags_line(&tags.join(" ")));
        }

        for l in entry.text.lines() {
            lines.push(highlight_tags_line(l));
        }

        lines.push(Line::from(Span::styled(
            "────────────",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        )));

        if breathe {
            let age = (chrono::Local::now() - entry.timestamp).num_milliseconds() as u128;
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
        let h = lines.len() as u16;
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

pub const MAX_BLOCK_LEN: usize = 180;

pub fn split_blocks(text: &str) -> Vec<String> {
    text.chars()
        .collect::<Vec<_>>()
        .chunks(MAX_BLOCK_LEN)
        .map(|c| c.iter().collect::<String>())
        .collect()
}

impl AppState {
    pub fn add_journal_text(&mut self, text: &str) {
        for block in split_blocks(text) {
            if block.trim().is_empty() {
                continue;
            }

            self.triage_capture_text(&block, crate::triage::logic::TriageSource::Zen);

            let entry = ZenJournalEntry {
                timestamp: chrono::Local::now(),
                text: block,
                prev_text: None,
            };

            self.zen_journal_entries.push(entry);
        }
    }

    pub fn edit_journal_entry(&mut self, index: usize, text: &str) {
        if let Some(entry) = self.zen_journal_entries.get_mut(index) {
            entry.prev_text = Some(entry.text.clone());
            entry.text = text.to_string();
        }
    }

    pub fn delete_journal_entry(&mut self, index: usize) {
        if index < self.zen_journal_entries.len() {
            self.zen_journal_entries.remove(index);
        }
    }

    pub fn focus_journal_entry(&mut self, index: usize) {
        if index < self.zen_journal_entries.len() {
            self.scroll_offset = index;
        }
    }

    pub fn start_edit_journal_entry(&mut self, index: usize) {
        if let Some(entry) = self.zen_journal_entries.get(index) {
            self.zen_draft.text = entry.text.clone();
            self.zen_draft.editing = Some(index);
        }
    }

    pub fn cancel_edit_journal_entry(&mut self) {
        self.zen_draft.editing = None;
        self.zen_draft.text.clear();
    }

    /// Return journal entries containing any of the provided tags (used by Triage).
    pub fn tagged_journal_entries(&self, tags: &[&str]) -> Vec<ZenJournalEntry> {
        self.zen_journal_entries
            .iter()
            .filter(|e| tags.iter().any(|t| e.text.contains(t)))
            .cloned()
            .collect()
    }

    /// Return journal entries that match the current active Zen tag filter.
    pub fn filtered_journal_entries(&self) -> Vec<&ZenJournalEntry> {
        self.zen_journal_entries
            .iter()
            .filter(|e| {
                if let Some(tag) = &self.zen_tag_filter {
                    extract_tags(&e.text)
                        .iter()
                        .any(|t| t.eq_ignore_ascii_case(tag))
                } else {
                    true
                }
            })
            .collect()
    }

    /// Set the active tag filter in Zen.
    pub fn set_tag_filter(&mut self, tag: Option<&str>) {
        self.zen_tag_filter = tag.map(|t| t.to_string());
    }

    /// Toggle between Journal, Daily summary, and Weekly summary modes.
    pub fn toggle_summary_view(&mut self) {
        use crate::state::{ZenSummaryMode, ZenViewMode};
        match self.zen_view_mode {
            ZenViewMode::Summary => {
                self.zen_summary_mode = match self.zen_summary_mode {
                    ZenSummaryMode::Daily => ZenSummaryMode::Weekly,
                    ZenSummaryMode::Weekly => {
                        self.zen_view_mode = ZenViewMode::Journal;
                        ZenSummaryMode::Daily
                    }
                };
            }
            _ => {
                self.zen_view_mode = ZenViewMode::Summary;
                self.zen_summary_mode = ZenSummaryMode::Daily;
            }
        }
    }
}
