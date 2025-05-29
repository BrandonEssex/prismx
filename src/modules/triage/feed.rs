use crate::plugins::api;
use crate::state::{AppState, ZenJournalEntry};
use crate::triage::state::{TriageEntry, TriageSource};

const TRIAGE_TAGS: &[&str] = &["#todo", "#triton", "#priority", "#now"];

fn has_triage_tag(tags: &[String]) -> bool {
    tags.iter().any(|t| TRIAGE_TAGS.contains(&t.as_str()))
}

/// Capture a single Zen journal entry into triage if it contains triage tags.
pub fn capture_zen_entry(state: &mut AppState, entry: &ZenJournalEntry) {
    if !has_triage_tag(&entry.tags) {
        return;
    }
    let exists = state.triage_entries.iter().any(|e| {
        e.source == TriageSource::Zen && e.created == entry.timestamp && e.text == entry.text
    });
    if exists {
        return;
    }
    let mut triage_entry = TriageEntry::new(state.triage_entries.len(), &entry.text, TriageSource::Zen);
    triage_entry.created = entry.timestamp;
    triage_entry.tags = entry.tags.clone();
    state.triage_entries.push(triage_entry);
}

/// Pull all tagged Zen entries into triage.
pub fn sync_from_zen(state: &mut AppState) {
    let entries: Vec<ZenJournalEntry> = state.zen_journal_entries.clone();
    for entry in entries {
        capture_zen_entry(state, &entry);
    }
}

/// Drain plugin-provided tasks into triage.
pub fn sync_from_plugins(state: &mut AppState) {
    for task in api::drain_tasks() {
        let text_lc = task.to_lowercase();
        if !TRIAGE_TAGS.iter().any(|t| text_lc.contains(t)) {
            continue;
        }
        if state.triage_entries.iter().any(|e| e.source == TriageSource::Spotlight && e.text == task) {
            continue;
        }
        let entry = TriageEntry::new(state.triage_entries.len(), &task, TriageSource::Spotlight);
        state.triage_entries.push(entry);
    }
}

/// Synchronize triage feed from all sources.
pub fn sync(state: &mut AppState) {
    sync_from_zen(state);
    sync_from_plugins(state);
}
