use crate::plugins::api;
use crate::state::{AppState, ZenJournalEntry};
use crate::triage::state::{TriageEntry, TriageSource};
use crate::triage::helpers::extract_tags;

use tracing::info;

const TRIAGE_TAGS: &[&str] = &["#todo", "#triton", "#priority", "#now"];

fn exists(state: &AppState, source: TriageSource, text: &str) -> bool {
    state
        .triage_entries
        .iter()
        .any(|e| e.source == source && e.text == text)
}

fn has_triage_tag(tags: &[String]) -> bool {
    tags.iter().any(|t| TRIAGE_TAGS.contains(&t.as_str()))
}

/// Capture a single Zen journal entry into triage if it contains triage tags.
pub fn capture_zen_entry(state: &mut AppState, entry: &ZenJournalEntry) {
    if !has_triage_tag(&entry.tags) {
        return;
    }
    if exists(state, TriageSource::Zen, &entry.text) {
        return;
    }
    let mut triage_entry = TriageEntry::new(state.triage_entries.len(), &entry.text, TriageSource::Zen);
    triage_entry.created = entry.timestamp;
    triage_entry.tags = entry.tags.clone();
    state.triage_entries.push(triage_entry);
}

/// Pull all tagged Zen entries into triage.
pub fn sync_from_zen(state: &mut AppState) {
    let mut added = false;
    let entries = state.zen_journal_entries.clone();
    for entry in entries {
        let prev_len = state.triage_entries.len();
        capture_zen_entry(state, &entry);
        if state.triage_entries.len() > prev_len {
            added = true;
        }
    }
    if added {
        state.triage_entries.sort_by(|a, b| b.created.cmp(&a.created));
    }
}

/// Drain plugin-provided tasks into triage.
pub fn sync_from_plugins(state: &mut AppState) {
    let mut added = false;
    for task in api::drain_tasks() {
        let tags = extract_tags(&task);
        if tags.is_empty() || !has_triage_tag(&tags) {
            continue;
        }
        if exists(state, TriageSource::Spotlight, &task) {
            continue;
        }
        let mut entry = TriageEntry::new(state.triage_entries.len(), &task, TriageSource::Spotlight);
        entry.tags = tags;
        state.triage_entries.push(entry);
        added = true;
    }
    if added {
        state.triage_entries.sort_by(|a, b| b.created.cmp(&a.created));
    }
}

/// Synchronize triage feed from all sources.
pub fn sync(state: &mut AppState) {
    sync_from_zen(state);
    sync_from_plugins(state);
}

/// Load sample feed items for debug preview.
pub fn load_sample(state: &mut AppState) {
    if !state.triage_entries.is_empty() {
        return;
    }

    let samples = [
        "[🔥] #now Fix crash on startup",
        "[🧠] #triton Refactor rendering engine",
        "[✅] #done Write unit tests",
        "[💡] #todo Research plugin API",
        "[📝] #todo Update documentation",
        "[🔍] #triton Investigate memory leak",
        "[🚀] #now Optimize build pipeline",
    ];

    for text in samples.iter() {
        let mut entry = TriageEntry::new(state.triage_entries.len(), text, TriageSource::Zen);
        entry.tags = extract_tags(text);
        state.triage_entries.push(entry);
    }

    info!("TRIAGE_FEED[✓]");
}
