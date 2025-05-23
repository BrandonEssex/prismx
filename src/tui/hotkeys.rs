use crossterm::event::{KeyCode, KeyModifiers};
use crate::state::AppState;

pub fn match_hotkey(action: &str, code: KeyCode, mods: KeyModifiers, state: &AppState) -> bool {
    // Normalize input: convert letter characters to lowercase so hotkey
    // matching is case-insensitive. Crossterm reports `KeyCode::Char` in the
    // case typed by the user (e.g. Ctrl+Shift+S yields `Char('S')`). Lowercase
    // before comparison so bindings using lowercase letters still match.
    let code_norm_char = match code {
        KeyCode::Char(c) => Some(c.to_ascii_lowercase()),
        _ => None,
    };

    if let Some(binding_raw) = state.hotkeys.get(action) {
        let binding = binding_raw
            .trim()
            .to_ascii_lowercase()
            .replace('–', "-")
            .replace('—', "-");

        let parts: Vec<&str> = binding.split('-').collect();
        let (m, k) = if parts.len() == 2 {
            (parts[0].trim(), parts[1].trim())
        } else {
            ("", parts[0].trim())
        };

        let mod_match = match m {
            "ctrl" => mods == KeyModifiers::CONTROL,
            "shift" => mods == KeyModifiers::SHIFT,
            "alt" => mods == KeyModifiers::ALT,
            "ctrl+shift" | "ctrl-shift" => {
                mods.contains(KeyModifiers::CONTROL) && mods.contains(KeyModifiers::SHIFT)
            }
            "" => mods.is_empty() || mods == KeyModifiers::NONE,
            _ => false,
        };


        let code_match = match k {
            "tab" => code == KeyCode::Tab,
            "shift-tab" => matches!(code, KeyCode::BackTab | KeyCode::Tab) && mods.contains(KeyModifiers::SHIFT),
            "enter" => code == KeyCode::Enter,
            "backspace" => code == KeyCode::Backspace,
            "esc" => code == KeyCode::Esc,
            "?" => code == KeyCode::Char('?'),
            "," => code == KeyCode::Char(','),
            "." => code == KeyCode::Char('.'),
            "d" => code_norm_char == Some('d'),
            "w" => code_norm_char == Some('w'),
            "q" => code_norm_char == Some('q'),
            "n" => code_norm_char == Some('n'),
            "t" => code_norm_char == Some('t'),
            "x" => code_norm_char == Some('x'),
            "c" => code_norm_char == Some('c'),
            "h" => code_norm_char == Some('h'),
            "e" => code_norm_char == Some('e'),
            "z" => code_norm_char == Some('z'),
            "y" => code_norm_char == Some('y'),
            "m" => code_norm_char == Some('m'),
            "space" => code == KeyCode::Char(' '),
            "r" => code_norm_char == Some('r'),
            "l" => code_norm_char == Some('l'),
            "g" => code_norm_char == Some('g'),
            "s" => code_norm_char == Some('s'),
            "o" => code_norm_char == Some('o'),


            _ => false,
        };

        mod_match && code_match
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctrl_shift_s_matches_lowercase_binding() {
        let state = AppState::default();
        let result = match_hotkey(
            "save_workspace",
            KeyCode::Char('S'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            &state,
        );
        assert!(result, "Ctrl+Shift+S should match save_workspace hotkey");
    }
}
