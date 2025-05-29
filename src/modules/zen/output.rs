use crate::state::{AppState, ZenJournalEntry};

const TRIAGE_TAGS: &[&str] = &["#todo", "#triton", "#priority", "#now"];

/// Return all journal entries containing triage-related tags.
pub fn tagged_entries(state: &AppState) -> Vec<ZenJournalEntry> {
    state
        .zen_journal_entries
        .iter()
        .filter(|e| e.tags.iter().any(|t| TRIAGE_TAGS.contains(&t.as_str())))
        .cloned()
        .collect()
}
