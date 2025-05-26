use crate::state::ZenJournalEntry;

/// Tags that mark an entry for triage.
const TRIAGE_TAGS: &[&str] = &["#TODO", "#TRITON", "#PRIORITY", "#NOW"];

/// Return journal entries that contain any of the triage tags.
/// Entries are sorted most recent first.
pub fn collect(entries: &[ZenJournalEntry]) -> Vec<ZenJournalEntry> {
    let mut filtered: Vec<ZenJournalEntry> = entries
        .iter()
        .filter(|e| TRIAGE_TAGS.iter().any(|tag| e.text.contains(tag)))
        .cloned()
        .collect();
    // Sort by timestamp descending
    filtered.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    filtered
}
