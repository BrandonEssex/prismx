use chrono::Local;
use crate::state::{AppState, ZenJournalEntry};

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
            if block.trim().is_empty() { continue; }
            let entry = ZenJournalEntry { timestamp: Local::now(), text: block };
            self.zen_journal_entries.push(entry);
        }
    }

    pub fn edit_journal_entry(&mut self, index: usize, text: &str) {
        if let Some(entry) = self.zen_journal_entries.get_mut(index) {
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

    pub fn filtered_journal_entries(&self) -> Vec<&ZenJournalEntry> {
        self.zen_journal_entries
            .iter()
            .filter(|e| {
                if let Some(tag) = &self.zen_tag_filter {
                    extract_tags(&e.text).iter().any(|t| t.eq_ignore_ascii_case(tag))
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn set_tag_filter(&mut self, tag: Option<&str>) {
        self.zen_tag_filter = tag.map(|t| t.to_string());
    }

    pub fn toggle_summary_view(&mut self) {
        self.zen_view_mode = match self.zen_view_mode {
            crate::state::ZenViewMode::Summary => crate::state::ZenViewMode::Journal,
            _ => crate::state::ZenViewMode::Summary,
        };
    }

}
