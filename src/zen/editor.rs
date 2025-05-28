use crossterm::event::KeyCode;
use crate::state::AppState;
use crate::state::ZenJournalEntry;

/// Handle key input for Zen compose mode.
pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Char(c) => {
            state.zen_draft.text.push(c);
        }
        KeyCode::Backspace => {
            state.zen_draft.text.pop();
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
            } else if let Some(idx) = state.zen_draft.editing {
                state.edit_journal_entry(idx, &text);
                state.zen_draft.editing = None;
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
                    };
                    state.zen_journal_entries.push(entry.clone());
                    state.append_journal_entry(&entry);
                }
                state.zen_draft.text.clear();
            }
        }
        _ => {}
    }
}
