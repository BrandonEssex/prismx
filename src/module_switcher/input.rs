use crate::hotkeys::match_hotkey;
use crate::shortcuts::{match_shortcut, Shortcut};

pub fn handle_input(key: &str) -> Option<String> {
    if let Some(action) = match_shortcut(key) {
        return Some(action);
    }

    if let Some(action) = match_hotkey(key) {
        return Some(action);
    }

    None
}
