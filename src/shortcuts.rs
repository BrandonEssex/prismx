use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shortcut {
    ZoomIn,
    ZoomOut,
    ToggleDebugInput,
    ToggleDebugBorder,
}

pub fn match_shortcut(code: KeyCode, mods: KeyModifiers) -> Option<Shortcut> {
    use KeyCode::*;
    use KeyModifiers as Mods;

    // Ctrl.+Shift.+D toggles debug input mode
    match (code, mods) {
        (Char('='), m) if m.contains(Mods::CONTROL) || m.contains(Mods::ALT) => Some(Shortcut::ZoomIn),
        (Char('+'), m) if m.contains(Mods::CONTROL) || m.contains(Mods::ALT) => Some(Shortcut::ZoomIn),
        (Char('-'), m) if m.contains(Mods::CONTROL) || m.contains(Mods::ALT) => Some(Shortcut::ZoomOut),
        (Char('d'), m) if m.contains(Mods::CONTROL) && m.contains(Mods::SHIFT) => Some(Shortcut::ToggleDebugInput),
        (Char('b'), m) if m.contains(Mods::CONTROL) && m.contains(Mods::SHIFT) => Some(Shortcut::ToggleDebugBorder),
        _ => None,
    }
}
