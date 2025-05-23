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
            "" => mods.is_empty() || mods == KeyModifiers::NONE,
            _ => false,
        };


        let code_lower_char = match code {
            KeyCode::Char(c) => Some(c.to_ascii_lowercase()),
            _ => None,
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
            "d" => code_lower_char == Some('d'),
            "w" => code_lower_char == Some('w'),
            "q" => code_lower_char == Some('q'),
            "n" => code_lower_char == Some('n'),
            "t" => code_lower_char == Some('t'),
            "x" => code_lower_char == Some('x'),
            "c" => code_lower_char == Some('c'),
            "h" => code_lower_char == Some('h'),
            "e" => code_lower_char == Some('e'),
            "z" => code_lower_char == Some('z'),
            "y" => code_lower_char == Some('y'),
            "m" => code_lower_char == Some('m'),
            "space" => code == KeyCode::Char(' '),
            "r" => code_lower_char == Some('r'),
            "l" => code_lower_char == Some('l'),
            "g" => code_lower_char == Some('g'),
            "s" => code_lower_char == Some('s'),
            "o" => code_lower_char == Some('o'),


            _ => false,
        };

        mod_match && code_match
    } else {
        false
    }
}
