use crossterm::event::{KeyCode, KeyModifiers};
use crate::state::AppState;

pub fn match_hotkey(action: &str, code: KeyCode, mods: KeyModifiers, state: &AppState) -> bool {
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
            "alt+shift" | "alt-shift" => {
                mods.contains(KeyModifiers::ALT) && mods.contains(KeyModifiers::SHIFT)
            }
            "" => mods.is_empty() || mods == KeyModifiers::NONE,
            _ => false,
        };


        let normalized_code = match code {
            KeyCode::Char(c) => KeyCode::Char(c.to_ascii_lowercase()),
            other => other,
        };

        let code_match = match k {
            "tab" => normalized_code == KeyCode::Tab,
            "shift-tab" => matches!(normalized_code, KeyCode::BackTab | KeyCode::Tab) && mods.contains(KeyModifiers::SHIFT),
            "enter" => normalized_code == KeyCode::Enter,
            "backspace" => normalized_code == KeyCode::Backspace,
            "esc" => normalized_code == KeyCode::Esc,
            "?" => normalized_code == KeyCode::Char('?'),
            "," => normalized_code == KeyCode::Char(','),
            "." => normalized_code == KeyCode::Char('.'),
            "d" => normalized_code == KeyCode::Char('d'),
            "w" => normalized_code == KeyCode::Char('w'),
            "q" => normalized_code == KeyCode::Char('q'),
            "n" => normalized_code == KeyCode::Char('n'),
            "t" => normalized_code == KeyCode::Char('t'),
            "x" => normalized_code == KeyCode::Char('x'),
            "c" => normalized_code == KeyCode::Char('c'),
            "h" => normalized_code == KeyCode::Char('h'),
            "e" => normalized_code == KeyCode::Char('e'),
            "z" => normalized_code == KeyCode::Char('z'),
            "y" => normalized_code == KeyCode::Char('y'),
            "m" => normalized_code == KeyCode::Char('m'),
            "space" => normalized_code == KeyCode::Char(' '),
            "r" => normalized_code == KeyCode::Char('r'),
            "l" => normalized_code == KeyCode::Char('l'),
            "g" => normalized_code == KeyCode::Char('g'),
            "s" => normalized_code == KeyCode::Char('s'),
            "o" => normalized_code == KeyCode::Char('o'),


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
    fn ctrl_shift_s_uppercase_matches() {
        let state = AppState::default();
        let result = match_hotkey(
            "save_workspace",
            KeyCode::Char('S'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            &state,
        );
        assert!(result, "Uppercase Ctrl+Shift+S should match save_workspace");
    }
}
