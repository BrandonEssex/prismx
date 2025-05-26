use chrono::Local;
use crate::state::{AppState, ZenJournalEntry};

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
            let entry = ZenJournalEntry { timestamp: Local::now(), text: block, prev_text: None };
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
}
