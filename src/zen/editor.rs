use crossterm::event::KeyCode;
use crate::state::AppState;
use crate::state::ZenJournalEntry;

/// Handle key input for Zen compose mode.
pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Char(c) => {
            // Spawn a new entry on first character when not editing
            if state.zen_draft.editing.is_none() && state.zen_draft.text.is_empty() {
                let entry = ZenJournalEntry {
                    timestamp: chrono::Local::now(),
                    text: String::new(),
                    prev_text: None,
                };
                state.zen_journal_entries.push(entry);
                state.zen_draft.editing = Some(state.zen_journal_entries.len() - 1);
            }
            state.zen_draft.text.push(c);
            if let Some(idx) = state.zen_draft.editing {
                let text = state.zen_draft.text.clone();
                state.edit_journal_entry(idx, &text);
            }

            // Auto-finalize when the user types the /send command
            if state.zen_draft.text.ends_with("/send") {
                let len = state.zen_draft.text.len();
                state.zen_draft.text.truncate(len.saturating_sub(5));
                if let Some(idx) = state.zen_draft.editing {
                    let text = state.zen_draft.text.clone();
                    state.edit_journal_entry(idx, &text);
                }
                finalize_entry(state);
            }
        }
        KeyCode::Backspace => {
            state.zen_draft.text.pop();
            if let Some(idx) = state.zen_draft.editing {
                let text = state.zen_draft.text.clone();
                state.edit_journal_entry(idx, &text);
            }
        }
        KeyCode::Enter => {
            finalize_entry(state);
        }
        _ => {}
    }
}

fn finalize_entry(state: &mut AppState) {
    let text = state.zen_draft.text.trim().to_string();

    if text.is_empty() {
        // Drop empty draft
        if let Some(idx) = state.zen_draft.editing {
            if let Some(entry) = state.zen_journal_entries.get(idx) {
                if entry.text.is_empty() {
                    state.zen_journal_entries.remove(idx);
                }
            }
        }
        state.zen_draft.text.clear();
        state.zen_draft.editing = None;
        return;
    }

    if let Some(idx) = state.zen_draft.editing {
        state.edit_journal_entry(idx, &text);
        let entry = state.zen_journal_entries[idx].clone();
        state.append_journal_entry(&entry);
    } else {
        let entry = ZenJournalEntry {
            timestamp: chrono::Local::now(),
            text: text.clone(),
            prev_text: None,
        };
        state.zen_journal_entries.push(entry.clone());
        state.append_journal_entry(&entry);
    }

    state.zen_draft.text.clear();
    state.zen_draft.editing = None;
}
