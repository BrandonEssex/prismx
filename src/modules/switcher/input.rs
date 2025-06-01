use crossterm::event::{KeyCode, KeyModifiers};
use crate::state::AppState;
use crate::modules::switcher::MODULES;

/// Handle key events for the app switcher overlay.
pub fn handle_key(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    if !state.module_switcher_open {
        if code == KeyCode::BackTab && mods.is_empty() {
            state.module_switcher_open = true;
            state.module_switcher_index = crate::modules::switcher::index_for_mode(&state.mode);
            return true;
        }
        return false;
    }

    let count = MODULES.len();
    match code {
        KeyCode::Left | KeyCode::BackTab => {
            state.module_switcher_index = (state.module_switcher_index + count - 1) % count;
            true
        }
        KeyCode::Right | KeyCode::Tab => {
            state.module_switcher_index = (state.module_switcher_index + 1) % count;
            true
        }
        KeyCode::Enter => {
            state.mode = crate::modules::switcher::mode_for_index(state.module_switcher_index).into();
            state.module_switcher_open = false;
            true
        }
        KeyCode::Esc => {
            state.module_switcher_open = false;
            true
        }
        _ => false,
    }
}
