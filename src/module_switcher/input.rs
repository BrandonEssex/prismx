use crossterm::event::{KeyCode, KeyModifiers};

use crate::state::AppState;
use crate::tui::hotkeys::match_hotkey;

use super::control;

/// Handle key events for the module switcher.
/// Returns `true` if the event was consumed.
pub fn handle_keys(state: &mut AppState, code: KeyCode, modifiers: KeyModifiers) -> bool {
    if state.module_switcher_open {
        match code {
            KeyCode::Tab => {
                control::next_module(state);
                tracing::debug!("[INPUT] module switcher index {}", state.module_switcher_index);
            }
            KeyCode::Enter => {
                control::select_current(state);
                tracing::info!("[INPUT] mode switched to {}", state.mode);
            }
            KeyCode::Esc => {
                control::close_switcher(state);
                tracing::debug!("[INPUT] module switcher closed");
            }
            _ => {}
        }
        return true;
    }

    if match_hotkey("open_module_switcher", code, modifiers, state) {
        control::open_switcher(state);
        tracing::info!("[INPUT] module switcher opened");
        return true;
    }
    false
}
