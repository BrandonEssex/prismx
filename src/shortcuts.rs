use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shortcut {
    ZoomIn,
    ZoomOut,
    ZoomReset,
    ToggleDebugInput,
}

pub fn match_shortcut(code: KeyCode, mods: KeyModifiers) -> Option<Shortcut> {
    use KeyCode::*;
    use KeyModifiers as Mods;

    // Ctrl.+Alt.+Z => Zoom In
    // Ctrl.+Alt.+X => Zoom Out
    // Ctrl.+Alt.+0 => Zoom Reset
    match (code, mods) {
        (Char('z'), m) if m.contains(Mods::CONTROL) && m.contains(Mods::ALT) => Some(Shortcut::ZoomIn),
        (Char('x'), m) if m.contains(Mods::CONTROL) && m.contains(Mods::ALT) => Some(Shortcut::ZoomOut),
        (Char('0'), m) if m.contains(Mods::CONTROL) && m.contains(Mods::ALT) => Some(Shortcut::ZoomReset),
        (Char('d'), m) if m.contains(Mods::CONTROL) && m.contains(Mods::SHIFT) => Some(Shortcut::ToggleDebugInput),
        _ => None,
    }
}
