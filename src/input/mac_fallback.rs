use crossterm::event::{KeyCode, KeyModifiers};
use crate::state::AppState;

/// Handle Cmd+Left/Right scrolling on macOS.
/// Returns true if the event was handled.
pub fn handle_cmd_arrows(_code: KeyCode, _modifiers: KeyModifiers, _state: &mut AppState) -> bool {
    #[cfg(target_os = "macos")]
    {
        if _modifiers.contains(KeyModifiers::SUPER) && _state.mode == "gemx" {
            match _code {
                KeyCode::Left => {
                    _state.scroll_x = _state.scroll_x.saturating_sub(4);
                    return true;
                }
                KeyCode::Right => {
                    _state.scroll_x = _state.scroll_x.saturating_add(4);
                    return true;
                }
                _ => {}
            }
        }
    }
    false
}
