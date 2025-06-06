use crossterm::event::KeyCode;
use crate::state::{AppState, ZenJournalEntry};
use crate::state::view::ZenLayoutMode;
use crate::state::ZenViewMode;
use crate::zen::utils::parse_tags;

/// Handle key input for Zen compose mode.
pub fn handle_key(state: &mut AppState, key: KeyCode) {
    tracing::info!("[ZEN] handle_key {:?}", key);
    if state.zen_layout_mode != ZenLayoutMode::Compose
        || state.zen_view_mode != ZenViewMode::Write
    {
        return;
    }

    if key != KeyCode::Enter {
        state.zen_draft.was_submitted = false;
    }

    match key {
        KeyCode::Char(c) => {
            // Spawn a new entry on first character when not editing
            if state.zen_draft.editing.is_none() && state.zen_draft.text.is_empty() {
                let entry = ZenJournalEntry {
                    timestamp: chrono::Local::now(),
                    text: String::new(),
                    prev_text: None,
                    tags: vec![],
                    frame: 0,
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
        KeyCode::Up => {
            if state.zen_draft.text.is_empty() && state.zen_draft.editing.is_none() {
                let len = state.zen_journal_entries.len();
                if len > 0 {
                    let idx = state.zen_history_index.unwrap_or(len).saturating_sub(1);
                    state.zen_history_index = Some(idx);
                    let desired = len.saturating_sub(1) - idx;
                    if state.scroll_offset < desired {
                        state.scroll_offset = desired;
                    }
                    clamp_scroll_limit(state, len);
                }
            }
        }
        KeyCode::Down => {
            if state.zen_draft.text.is_empty() && state.zen_draft.editing.is_none() {
                if let Some(mut idx) = state.zen_history_index {
                    let len = state.zen_journal_entries.len();
                    if idx + 1 < len {
                        idx += 1;
                        state.zen_history_index = Some(idx);
                        let desired = len.saturating_sub(1) - idx;
                        if state.scroll_offset > desired {
                            state.scroll_offset = desired;
                        }
                        clamp_scroll_limit(state, len);
                    } else {
                        state.zen_history_index = None;
                        state.scroll_offset = 0;
                        clamp_scroll_limit(state, len);
                    }
                }
            }
        }
        KeyCode::Esc => {
            state.cancel_edit_journal_entry();
            state.zen_history_index = None;
            state.zen_draft.text.clear();
            state.zen_draft.was_submitted = false;
        }
        KeyCode::Backspace => {
            state.zen_draft.text.pop();
            if let Some(idx) = state.zen_draft.editing {
                let text = state.zen_draft.text.clone();
                state.edit_journal_entry(idx, &text);
            }
        }
        KeyCode::Enter => {
            if state.zen_draft.was_submitted {
                return;
            }
            let text = state.zen_draft.text.trim().to_string();
            if text.is_empty() && state.zen_draft.editing.is_none() {
                if let Some(idx) = state.zen_history_index.take() {
                    state.start_edit_journal_entry(idx);
                    let len = state.zen_journal_entries.len();
                    state.scroll_offset = len.saturating_sub(1) - idx;
                    clamp_scroll_limit(state, len);
                }
            } else if text == "/scroll" {
                crate::ui::input::toggle_zen_view(state);
                state.zen_draft.text.clear();
            } else if text.starts_with("/edit ") {
                if let Ok(idx) = text[6..].trim().parse::<usize>() {
                    state.start_edit_journal_entry(idx);
                    let len = state.zen_journal_entries.len();
                    state.scroll_offset = len.saturating_sub(1) - idx;
                    clamp_scroll_limit(state, len);
                }
                state.zen_draft.text.clear();
            } else if text == "/cancel" {
                state.cancel_edit_journal_entry();
                state.zen_draft.text.clear();
            } else if text == "/plugins" {
                use crate::plugin::registry;
                let list = registry::registry();
                if list.is_empty() {
                    state.status_message = "No plugins loaded".into();
                } else {
                    let lines: Vec<String> = list
                        .iter()
                        .map(|p| {
                            if let Some(ref path) = p.path {
                                format!("{} v{} ({})", p.name, p.version, path)
                            } else {
                                format!("{} v{}", p.name, p.version)
                            }
                        })
                        .collect();
                    state.status_message = lines.join(", ");
                }
                state.zen_draft.text.clear();
            } else if state.zen_draft.editing.is_some() {
                finalize_entry(state);
            } else if !text.is_empty() {
                if crate::config::theme::ThemeConfig::load().zen_breathe() {
                    std::thread::sleep(std::time::Duration::from_millis(150));
                }
                let entry = ZenJournalEntry {
                    timestamp: chrono::Local::now(),
                    text: text.clone(),
                    prev_text: None,
                    frame: 0,
                    tags: parse_tags(&text),
                };
                state.zen_journal_entries.push(entry.clone());
                state.append_journal_entry(&entry);
                state.zen_draft.text.clear();
                state.zen_draft.editing = None;
                // Auto-scroll to show the newly added entry
                state.scroll_offset = 0;
                clamp_scroll_limit(state, state.zen_journal_entries.len());
            } else {
                finalize_entry(state);
            }

            state.status_message_last_updated = Some(std::time::Instant::now());
            state.zen_draft.was_submitted = true;
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
        crate::modules::triage::feed::capture_zen_entry(state, &entry);
        let len = state.zen_journal_entries.len();
        state.scroll_offset = len.saturating_sub(1) - idx;
        clamp_scroll_limit(state, len);
    } else {
        let entry = ZenJournalEntry {
            timestamp: chrono::Local::now(),
            text: text.clone(),
            prev_text: None,
            tags: parse_tags(&text),
            frame: 0,
        };
        state.zen_journal_entries.push(entry.clone());
        state.append_journal_entry(&entry);
        crate::modules::triage::feed::capture_zen_entry(state, &entry);
        state.scroll_offset = 0;
        clamp_scroll_limit(state, state.zen_journal_entries.len());
    }

    state.zen_draft.text.clear();
    state.zen_draft.editing = None;
    state.zen_draft.was_submitted = true;
}

/// Clamp scroll offset so it never exceeds the final entry index minus
/// the available window height.
fn clamp_scroll_limit(state: &mut AppState, len: usize) {
    use crossterm::terminal::size;
    let (_, h) = size().unwrap_or((80, 20));
    let window_height = h as usize;
    let max_offset = len.saturating_sub(window_height);
    if state.scroll_offset > max_offset {
        state.scroll_offset = max_offset;
    }
    // scroll_offset should never be negative but saturating_sub ensures it.
}
