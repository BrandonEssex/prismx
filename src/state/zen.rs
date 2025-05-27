// src/state/zen.rs
use super::core::{AppState, ZenJournalEntry, ZenTheme};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use dirs;

impl AppState {
    pub fn export_zen_to_file(&self) {
        let path = dirs::document_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("prismx")
            .join("zen_export.md");

        let content = self.zen_buffer.join("\n");

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(content.as_bytes());
        }
    }

    pub fn open_zen_file(&mut self, path: &str) {
        if let Ok(content) = std::fs::read_to_string(path) {
            self.zen_buffer = content.lines().map(|l| l.to_string()).collect();
            self.zen_buffer.push(String::new());
            self.zen_current_filename = path.to_string();
            self.zen_current_syntax = Self::syntax_from_extension(path);
            self.update_zen_word_count();
            self.zen_dirty = false;
            self.add_recent_file(path);
            self.zen_last_saved = Some(std::time::Instant::now());
        }
    }

    pub fn save_zen_file(&mut self, path: &str) {
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(mut file) = std::fs::File::create(path) {
            let _ = file.write_all(self.zen_buffer.join("\n").as_bytes());
            self.add_recent_file(path);
            self.zen_current_filename = path.to_string();
            self.zen_dirty = false;
            self.zen_last_saved = Some(std::time::Instant::now());
        }
    }

    pub fn update_zen_word_count(&mut self) {
        let text = self.zen_buffer.join(" ");
        self.zen_word_count = text.split_whitespace().count();
        crate::log_debug!(self, "Word count updated: {}", self.zen_word_count);
    }

    pub fn add_recent_file(&mut self, path: &str) {
        if let Some(pos) = self.zen_recent_files.iter().position(|p| p == path) {
            self.zen_recent_files.remove(pos);
        }
        self.zen_recent_files.insert(0, path.to_string());
        while self.zen_recent_files.len() > 5 {
            self.zen_recent_files.pop();
        }
    }

    pub fn zen_toolbar_len(&self) -> usize {
        3 + self.zen_recent_files.len()
    }

    pub fn zen_toolbar_handle_key(&mut self, key: crossterm::event::KeyCode) {
        let len = self.zen_toolbar_len();
        match key {
            crossterm::event::KeyCode::Up => {
                if self.zen_toolbar_index == 0 {
                    self.zen_toolbar_index = len.saturating_sub(1);
                } else {
                    self.zen_toolbar_index -= 1;
                }
            }
            crossterm::event::KeyCode::Down => {
                self.zen_toolbar_index = (self.zen_toolbar_index + 1) % len;
            }
            crossterm::event::KeyCode::Enter => {
                match self.zen_toolbar_index {
                    0 => {
                        self.zen_buffer = vec![String::new()];
                        self.zen_current_filename = "Untitled".into();
                        self.update_zen_word_count();
                        self.zen_dirty = false;
                    }
                    1 => {
                        if let Some(path) = self.zen_recent_files.first().cloned() {
                            self.open_zen_file(&path);
                        }
                    }
                    2 => {
                        if let Some(path) = self.zen_recent_files.first().cloned() {
                            self.save_zen_file(&path);
                        }
                    }
                    idx => {
                        if let Some(path) = self.zen_recent_files.get(idx - 3).cloned() {
                            self.open_zen_file(&path);
                        }
                    }
                }
                self.zen_toolbar_open = false;
            }
            _ => {}
        }
    }

    pub fn auto_save_zen(&mut self) {
        if self.zen_dirty {
            let should_save = self
                .zen_last_saved
                .map_or(true, |t| t.elapsed().as_secs() > 10);
            if should_save {
                let _ = std::fs::write(&self.zen_current_filename, self.zen_buffer.join("\n"));
                self.zen_last_saved = Some(std::time::Instant::now());
                self.zen_dirty = false;
            }
        }
    }

    pub fn load_today_journal(&mut self) {
        let path = format!("journals/{}.prismx", chrono::Local::now().format("%Y-%m-%d"));
        if let Ok(content) = fs::read_to_string(&path) {
            self.zen_journal_entries = content
                .lines()
                .filter_map(|line| {
                    let (ts, text) = line.split_once('|')?;
                    chrono::DateTime::parse_from_rfc3339(ts)
                        .ok()
                        .map(|dt| ZenJournalEntry {
                            timestamp: dt.with_timezone(&chrono::Local),
                            text: text.to_string(),
                            prev_text: None,
                        })
                })
                .collect();
        }
    }

    pub fn append_journal_entry(&mut self, entry: &ZenJournalEntry) {
        let _ = std::fs::create_dir_all("journals");
        let path = format!("journals/{}.prismx", chrono::Local::now().format("%Y-%m-%d"));
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(file, "{}|{}", entry.timestamp.to_rfc3339(), entry.text);
        }
    }

    pub fn cycle_zen_theme(&mut self) {
        self.zen_theme = match self.zen_theme {
            ZenTheme::DarkGray => ZenTheme::Light,
            ZenTheme::Light => ZenTheme::HighContrast,
            ZenTheme::HighContrast => ZenTheme::DarkGray,
        };
    }

    // PATCHED: Required methods from zen::journal.rs
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

    pub fn edit_journal_entry(&mut self, index: usize, text: &str) {
        if let Some(entry) = self.zen_journal_entries.get_mut(index) {
            entry.prev_text = Some(entry.text.clone());
            entry.text = text.to_string();
        }
    }

    pub fn filtered_journal_entries(&self) -> Vec<&ZenJournalEntry> {
        use crate::zen::utils::extract_tags;
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
