use crossterm::event::{KeyCode, KeyModifiers};
use crate::state::AppState;
use std::time::Instant;

pub fn match_hotkey(action: &str, code: KeyCode, mods: KeyModifiers, state: &AppState) -> bool {
    let code = match code {
        KeyCode::Char(c) => KeyCode::Char(c.to_ascii_lowercase()),
        _ => code,
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
            "alt+shift" | "alt-shift" => {
                mods.contains(KeyModifiers::ALT) && mods.contains(KeyModifiers::SHIFT)
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
            "m" => code == KeyCode::Char('m'),
            "s" => code == KeyCode::Char('s'),
            "o" => code == KeyCode::Char('o'),
            "space" => code == KeyCode::Char(' '),
            "r" => code == KeyCode::Char('r'),
            "l" => code == KeyCode::Char('l'),
            "g" => code == KeyCode::Char('g'),


            _ => false,
        };

        mod_match && code_match
    } else {
        false
    }
}

pub fn debug_input(state: &mut AppState, code: KeyCode, mods: KeyModifiers) {
    let mut parts = Vec::new();
    if mods.contains(KeyModifiers::CONTROL) { parts.push("Ctrl"); }
    if mods.contains(KeyModifiers::ALT) { parts.push("Alt"); }
    if mods.contains(KeyModifiers::SHIFT) { parts.push("Shift"); }

    let key_str = match code {
        KeyCode::Char(c) => c.to_string(),
        other => format!("{:?}", other),
    };

    let prefix = if parts.is_empty() { String::new() } else { format!("{}+", parts.join("+")) };
    state.status_message = format!("Key: {}{}", prefix, key_str);
    state.status_message_last_updated = Some(Instant::now());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctrl_shift_s_matches_lowercase_binding() {
        let mut state = AppState::default();
        state.hotkeys.insert("save_ws".into(), "ctrl+shift-s".into());

        assert!(match_hotkey(
            "save_ws",
            KeyCode::Char('S'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            &state,
        ));
    }
}
