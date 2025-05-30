use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use std::time::Instant;
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
        // Alt+Left: previous sibling
        KeyCode::Left if mods == KeyModifiers::ALT => {
            state.focus_prev_sibling();
            true
        }
        // Alt+Right: next sibling
        KeyCode::Right if mods == KeyModifiers::ALT => {
            state.focus_next_sibling();
            true
        }
        // Alt+Up: move to parent
        KeyCode::Up if mods == KeyModifiers::ALT => {
            state.move_focus_left();
            true
        }
        // Alt+Down: move to first child
        KeyCode::Down if mods == KeyModifiers::ALT => {
            state.move_focus_right();
            true
        }
        // Ctrl+Shift+Up: promote selected node
        KeyCode::Up if mods.contains(KeyModifiers::CONTROL) && mods.contains(KeyModifiers::SHIFT) => {
            if let Some(id) = state.selected {
                state.push_undo();
                crate::modules::gemx::logic::promote(&mut state.nodes, &mut state.root_nodes, id);
                state.selection_trail.push_back((id, Instant::now()));
                if state.selection_trail.len() > 8 { state.selection_trail.pop_front(); }
                state.status_message = "Node promoted".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Ctrl+Shift+Down: demote selected node under previous sibling
        KeyCode::Down if mods.contains(KeyModifiers::CONTROL) && mods.contains(KeyModifiers::SHIFT) => {
            if let Some(id) = state.selected {
                state.push_undo();
                crate::modules::gemx::logic::demote_prev_sibling(&mut state.nodes, &mut state.root_nodes, id);
                state.selection_trail.push_back((id, Instant::now()));
                if state.selection_trail.len() > 8 { state.selection_trail.pop_front(); }
                state.status_message = "Node demoted".into();
                state.status_message_last_updated = Some(Instant::now());
            }
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
