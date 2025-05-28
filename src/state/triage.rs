use crate::triage::state::{capture_entry, handle_inline_command, TriageSource};
use super::core::AppState;

impl AppState {
    /// Capture potential triage entry from text.
    pub fn triage_capture_text(&mut self, text: &str, source: TriageSource) {
        if handle_inline_command(self, text) { return; }
        capture_entry(self, source, text);
    }
}
