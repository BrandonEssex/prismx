use crossterm::event::{KeyCode, KeyModifiers};
use crate::state::AppState;

/// Handle Cmd+Left/Right scrolling on macOS.
/// Returns true if the event was handled.
pub fn handle_cmd_arrows(code: KeyCode, modifiers: KeyModifiers, state: &mut AppState) -> bool {
    #[cfg(target_os = "macos")]
    {
        if modifiers.contains(KeyModifiers::SUPER) && state.mode == "gemx" {
            match code {
                KeyCode::Left => {
                    state.scroll_x = state.scroll_x.saturating_sub(4);
                    return true;
                }
                KeyCode::Right => {
                    state.scroll_x = state.scroll_x.saturating_add(4);
                    return true;
                }
                _ => {}
            }
        }
    }
    false
}
