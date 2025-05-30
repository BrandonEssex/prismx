use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use crate::state::AppState;

/// Handle GemX (mindmap) specific keyboard shortcuts.
pub fn handle_key(state: &mut AppState, code: KeyCode, mods: KeyModifiers) -> bool {
    match code {
        // Create a free node with Ctrl+N only (no Shift)
        KeyCode::Char('n') if mods == KeyModifiers::CONTROL => {
            state.push_undo();
            crate::gemx::interaction::spawn_free_node(state);
            true
        }
        // Insert sibling on Enter
        KeyCode::Enter if mods.is_empty() => {
            state.push_undo();
            state.handle_enter_key();
            true
        }
        // Insert child on Tab
        KeyCode::Tab if mods.is_empty() => {
            state.push_undo();
            state.handle_tab_key();
            true
        }
        // Promote or free on Shift+Tab
        KeyCode::BackTab if mods.is_empty() => {
            state.push_undo();
            state.handle_shift_tab_key();
            true
        }
        _ => false,
    }
}

/// Handle GemX mouse interactions. Currently unused.
pub fn handle_mouse(_state: &mut AppState, _me: MouseEvent) -> bool {
    match _me.kind {
        MouseEventKind::Down(MouseButton::Left)
        | MouseEventKind::Drag(MouseButton::Left)
        | MouseEventKind::Up(MouseButton::Left) => {}
        _ => {}
    }
    false
}
