use crossterm::event::{KeyCode, KeyModifiers};

use crate::state::AppState;

/// Handle key events when the log viewer is active.
/// Returns true if the input was processed.
pub fn handle_log_keys(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    match code {
        KeyCode::Up => {
            if state.logs_scroll + 1 < 1000 { // arbitrary cap
                state.logs_scroll += 1;
            }
            true
        }
        KeyCode::Down => {
            if state.logs_scroll > 0 {
                state.logs_scroll -= 1;
            }
            true
        }
        KeyCode::Char(c) if mods.is_empty() || mods == KeyModifiers::SHIFT => {
            state.logs_filter.push(c);
            true
        }
        KeyCode::Backspace => {
            state.logs_filter.pop();
            true
        }
        _ => false,
    }
}

/// Toggle Zen Write/Review view.
pub fn toggle_zen_view(state: &mut AppState) {
    state.zen_view_mode = match state.zen_view_mode {
        crate::state::ZenViewMode::Write => crate::state::ZenViewMode::Review,
        crate::state::ZenViewMode::Review => crate::state::ZenViewMode::Write,
    };
}
