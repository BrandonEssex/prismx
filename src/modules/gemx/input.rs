use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use std::time::Instant;
use crate::state::AppState;
use crate::gemx::interaction::{self, node_at_position, start_drag as start_move,
    drag_update, end_drag};
use crate::modules::gemx::logic;

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
            if state.can_insert_node() {
                state.push_undo();
                state.handle_enter_key();
            } else {
                state.status_message = "Cannot insert: empty node".into();
                state.status_message_last_updated = Some(Instant::now());
            }
            true
        }
        // Insert child on Tab
        KeyCode::Tab if mods.is_empty() => {
            if state.can_insert_node() {
                state.push_undo();
                state.handle_tab_key();
            } else {
                state.status_message = "Cannot insert: empty node".into();
                state.status_message_last_updated = Some(Instant::now());
            }
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
pub fn handle_mouse(state: &mut AppState, me: MouseEvent) -> bool {
    match me.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(id) = node_at_position(state, me.column, me.row) {
                state.push_undo();
                start_move(state, id, me.column, me.row);
                state.drag_hover_target = None;
                state.start_drag();
                return true;
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if state.dragging.is_some() {
                drag_update(state, me.column, me.row);
                if let Some(tid) = node_at_position(state, me.column, me.row) {
                    if Some(tid) != state.dragging {
                        state.drag_hover_target = Some(tid);
                    } else {
                        state.drag_hover_target = None;
                    }
                } else {
                    state.drag_hover_target = None;
                }
                return true;
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            if let Some(id) = state.dragging {
                end_drag(state);
                let target = state.drag_hover_target.take();
                logic::reparent(&mut state.nodes, &mut state.root_nodes, id, target);
                logic::adopt_orphans(&mut state.nodes, &mut state.root_nodes);
                return true;
            }
        }
        _ => {}
    }
    false
}
