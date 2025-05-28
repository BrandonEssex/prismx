use crossterm::event::KeyCode;
use crate::state::AppState;
use crate::state::ZenJournalEntry;
use crate::zen::utils::parse_tags;

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
                    tags: vec![],
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
            let text = state.zen_draft.text.trim().to_string();
            if text == "/scroll" {
                crate::ui::input::toggle_zen_view(state);
                state.zen_draft.text.clear();
            } else if text.starts_with("/edit ") {
                if let Ok(idx) = text[6..].trim().parse::<usize>() {
                    state.start_edit_journal_entry(idx);
                }
                state.zen_draft.text.clear();
            } else if text == "/cancel" {
                state.cancel_edit_journal_entry();
                state.zen_draft.text.clear();
            } else {
                if !text.is_empty() {
                    if crate::config::theme::ThemeConfig::load().zen_breathe() {
                        std::thread::sleep(std::time::Duration::from_millis(150));
                    }
                    let entry = ZenJournalEntry {
                        timestamp: chrono::Local::now(),
                        text: text.clone(),
                        prev_text: None,
                        frame: 0,
                        tags: crate::zen::utils::parse_tags(&text),
                        tags: parse_tags(&text),
                    };
                    state.zen_journal_entries.push(entry.clone());
                    state.append_journal_entry(&entry);
                }
                state.zen_draft.text.clear();
                state.zen_draft.editing = None;
            }
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
            tags: parse_tags(&text),
        };
        state.zen_journal_entries.push(entry.clone());
        state.append_journal_entry(&entry);
    }

    state.zen_draft.text.clear();
    state.zen_draft.editing = None;
}
