use chrono::{DateTime, Local};
use crate::state::{AppState, ZenJournalEntry};
use crate::triage::helpers::extract_tags;

/// Tags that mark an entry for triage.
const TRIAGE_TAGS: &[&str] = &["#todo", "#triton", "#priority", "#now"];

#[derive(Clone, Debug, PartialEq)]
pub enum TriageSource {
    Zen,
    Gemx,
    Spotlight,
}

#[derive(Clone, Debug)]
pub struct TriageEntry {
    pub id: usize,
    pub text: String,
    pub tags: Vec<String>,
    pub source: TriageSource,
    pub created: DateTime<Local>,
    pub resolved: bool,
    pub archived: bool,
}

impl TriageEntry {
    pub fn new(id: usize, text: &str, source: TriageSource) -> Self {
        let tags = extract_tags(text);
        Self {
            id,
            text: text.to_string(),
            tags,
            source,
            created: Local::now(),
            resolved: false,
            archived: false,
        }
    }
}

/// Collect raw journal entries containing any triage tags.
pub fn collect(entries: &[ZenJournalEntry]) -> Vec<ZenJournalEntry> {
    let mut filtered: Vec<ZenJournalEntry> = entries
        .iter()
        .filter(|e| e.tags.iter().any(|tag| TRIAGE_TAGS.contains(&tag.as_str())))
        .cloned()
        .collect();

    // Sort most recent first
    filtered.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    filtered
}

pub fn capture_entry(state: &mut AppState, source: TriageSource, text: &str) {
    let text_lc = text.to_lowercase();
    if TRIAGE_TAGS.iter().any(|tg| text_lc.contains(tg)) {
        let id = state.triage_entries.len();
        let entry = TriageEntry::new(id, text, source);
        state.triage_entries.push(entry);
    }
}

pub fn handle_inline_command(state: &mut AppState, text: &str) -> bool {
    let mut parts = text.trim().split_whitespace();
    let cmd = parts.next().unwrap_or("");
    match cmd {
        "/resolve" => {
            if let Some(id) = parts.next().and_then(|v| v.parse::<usize>().ok()) {
                resolve(state, id);
            }
            true
        }
        "/dismiss" => {
            if let Some(id) = parts.next().and_then(|v| v.parse::<usize>().ok()) {
                dismiss(state, id);
            }
            true
        }
        "/archive" => {
            if let Some(id) = parts.next().and_then(|v| v.parse::<usize>().ok()) {
                archive(state, id);
            }
            true
        }
        _ => false,
    }
}

pub fn resolve(state: &mut AppState, id: usize) {
    if let Some(entry) = state.triage_entries.get_mut(id) {
        entry.resolved = true;
        if entry.tags.contains(&"#triton".to_string()) {
            entry.tags.retain(|t| t != "#triton");
            entry.tags.push("#done".into());
        } else if entry.tags.contains(&"#now".to_string()) {
            entry.tags.retain(|t| t != "#now");
            entry.tags.push("#triton".into());
        }
    }
}

pub fn dismiss(state: &mut AppState, id: usize) {
    if let Some(entry) = state.triage_entries.get_mut(id) {
        entry.archived = true;
    }
}

pub fn archive(state: &mut AppState, id: usize) {
    if let Some(entry) = state.triage_entries.get_mut(id) {
        entry.archived = true;
        entry.resolved = true;
    }
}

/// Simple pipeline updates. Entries tagged #NOW move to #TRITON after ~60s.
pub fn update_pipeline(state: &mut AppState) {
    for entry in &mut state.triage_entries {
        // Promote resolved #TRITON entries to #DONE
        if entry.resolved && entry.tags.contains(&"#triton".to_string()) {
            entry.tags.retain(|t| t != "#triton");
            entry.tags.push("#done".into());
            continue;
        }

        if entry.resolved {
            continue;
        }

        // Auto-promote #NOW to #TRITON after ~60s
        if entry.tags.contains(&"#now".to_string()) {
            let since = Local::now().signed_duration_since(entry.created).num_seconds();
            if since > 60 {
                entry.tags.retain(|t| t != "#now");
                entry.tags.push("#triton".into());
            }
        }
    }
}

/// Count visible triage tags for summary bar.
pub fn tag_counts(state: &AppState) -> (usize, usize, usize) {
    let mut now = 0usize;
    let mut triton = 0usize;
    let mut done = 0usize;
    for entry in &state.triage_entries {
        if entry.archived {
            continue;
        }
        if entry.tags.iter().any(|t| t.eq("#now")) {
            now += 1;
        }
        if entry.tags.iter().any(|t| t.eq("#triton")) {
            triton += 1;
        }
        if entry.tags.iter().any(|t| t.eq("#done")) {
            done += 1;
        }
    }
    (now, triton, done)
}
