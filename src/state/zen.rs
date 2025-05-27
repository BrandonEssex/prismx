// src/state/zen.rs
use chrono::Local;
use crate::state::{AppState, ZenJournalEntry};

/// Hard limit for block-based entries
pub const MAX_BLOCK_LEN: usize = 180;

/// Split user input into blocks of N characters
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
                timestamp: Local::now(),
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

    pub fn tagged_journal_entries(&self, tags: &[&str]) -> Vec<ZenJournalEntry> {
        self.zen_journal_entries
            .iter()
            .filter(|e| tags.iter().any(|t| e.text.contains(t)))
            .cloned()
            .collect()
    }

    pub fn filtered_journal_entries(&self) -> Vec<&ZenJournalEntry> {
        use crate::zen::utils::extract_tags;
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

    pub fn set_tag_filter(&mut self, tag: Option<&str>) {
        self.zen_tag_filter = tag.map(|t| t.to_string());
    }

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
