use crate::triage::state::{capture_entry, handle_inline_command, TriageSource};
use super::core::AppState;

impl AppState {
    /// Capture potential triage entry from text.
    pub fn triage_capture_text(&mut self, text: &str, source: TriageSource) {
        if handle_inline_command(self, text) { return; }
        capture_entry(self, source, text);
    }

    /// Move triage focus to previous visible entry.
    pub fn triage_focus_prev(&mut self) {
        let visible: Vec<usize> = self
            .triage_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.archived)
            .map(|(i, _)| i)
            .collect();
        if visible.is_empty() { return; }
        let current_pos = visible
            .iter()
            .position(|&i| i == self.triage_focus_index)
            .unwrap_or(0);
        let new_pos = if current_pos == 0 { visible.len() - 1 } else { current_pos - 1 };
        self.triage_focus_index = visible[new_pos];
    }

    /// Move triage focus to next visible entry.
    pub fn triage_focus_next(&mut self) {
        let visible: Vec<usize> = self
            .triage_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.archived)
            .map(|(i, _)| i)
            .collect();
        if visible.is_empty() { return; }
        let current_pos = visible
            .iter()
            .position(|&i| i == self.triage_focus_index)
            .unwrap_or(0);
        let new_pos = (current_pos + 1) % visible.len();
        self.triage_focus_index = visible[new_pos];
    }

    /// Archive the currently focused triage entry.
    pub fn triage_delete_current(&mut self) {
        if let Some(entry) = self.triage_entries.get_mut(self.triage_focus_index) {
            entry.archived = true;
        }
        // move focus to next available entry
        self.triage_focus_next();
    }

    /// Toggle a tag on the currently focused entry.
    pub fn triage_toggle_tag(&mut self, tag: &str) {
        if let Some(entry) = self.triage_entries.get_mut(self.triage_focus_index) {
            let has_tag = entry.tags.iter().any(|t| t.eq_ignore_ascii_case(tag));
            if has_tag {
                entry.tags.retain(|t| !t.eq_ignore_ascii_case(tag));
                let words: Vec<String> = entry
                    .text
                    .split_whitespace()
                    .filter(|w| !w.eq_ignore_ascii_case(tag))
                    .map(|w| w.to_string())
                    .collect();
                entry.text = words.join(" ");
            } else {
                if !entry.text.is_empty() && !entry.text.ends_with(' ') {
                    entry.text.push(' ');
                }
                entry.text.push_str(tag);
                entry.tags.push(tag.to_lowercase());
            }
        }
    }

    /// Update cached tag counts used in status views.
    pub fn triage_recalc_counts(&mut self) {
        let (n, t, d) = crate::triage::state::tag_counts(self);
        self.triage_summary.now = n;
        self.triage_summary.triton = t;
        self.triage_summary.done = d;
    }
}
