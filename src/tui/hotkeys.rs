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
            "ctrl" => mods.contains(KeyModifiers::CONTROL),
            "alt" => mods.contains(KeyModifiers::ALT),
            "shift" => mods.contains(KeyModifiers::SHIFT),
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
            "d" => code == KeyCode::Char('d'),
            "w" => code == KeyCode::Char('w'),
            "q" => code == KeyCode::Char('q'),
            "n" => code == KeyCode::Char('n'),
            "t" => code == KeyCode::Char('t'),
            "x" => code == KeyCode::Char('x'),
            "c" => code == KeyCode::Char('c'),
            "h" => code == KeyCode::Char('h'),
            "e" => code == KeyCode::Char('e'),
            "z" => code == KeyCode::Char('z'),
            "y" => code == KeyCode::Char('y'),
            _ => false,
        };

        mod_match && code_match
    } else {
        false
    }
}
