use crate::state::AppState;

pub fn create_child(state: &mut AppState) {
    state.push_undo();
    state.handle_tab_key();
}

pub fn create_sibling(state: &mut AppState) {
    state.push_undo();
    state.handle_enter_key();
}

pub fn add_free_node(state: &mut AppState) {
    state.push_undo();
    crate::gemx::interaction::spawn_free_node(state);
}

pub fn delete_selected(state: &mut AppState) {
    state.push_undo();
    state.delete_node();
}

pub fn undo(state: &mut AppState) {
    state.undo();
}

pub fn redo(state: &mut AppState) {
    state.redo();
}

pub fn open_module_switcher(state: &mut AppState) {
    state.module_switcher_open = true;
    state.module_switcher_index = crate::modules::switcher::index_for_mode(&state.mode);
}

pub fn start_drag(state: &mut AppState) {
    if state.selected_drag_source.is_some() {
        if let Some(target) = state.selected {
            state.push_undo();
            state.complete_drag(target);
        }
    } else {
        state.start_drag();
    }
}

pub fn start_link(state: &mut AppState) {
    if state.selected_drag_source.is_some() {
        if let Some(target) = state.selected {
            state.complete_link(target);
        }
    } else {
        state.start_link();
    }
}

pub fn toggle_link_mode(state: &mut AppState) {
    state.link_mode = !state.link_mode;
    crate::log_debug!(state, "link_mode toggled: {}", state.link_mode);
}

pub fn save(state: &mut AppState) {
    state.export_zen_to_file();
}

pub fn mode_zen(state: &mut AppState) {
    state.mode = "zen".into();
    state.zen_layout_mode = crate::state::view::ZenLayoutMode::Compose;
    state.zen_view_mode = crate::state::ZenViewMode::Write;
    state.scroll_offset = 0;
}

pub fn zen_toggle_theme(state: &mut AppState) {
    state.cycle_zen_theme();
}

pub fn debug_snapshot(state: &mut AppState) {
    crate::ui::components::debug::write_debug_snapshot(state);
}

pub fn debug_overlay(state: &mut AppState) {
    state.debug_overlay = !state.debug_overlay;
}

pub fn debug_overlay_sticky(state: &mut AppState) {
    state.debug_overlay_sticky = !state.debug_overlay_sticky;
    state.debug_overlay = state.debug_overlay_sticky;
}

pub fn reload_plugins() {
    crate::state::init::reload_plugins();
}

pub fn toggle_collapsed(state: &mut AppState) {
    state.toggle_collapse();
}

pub fn drill_down(state: &mut AppState) {
    state.drawing_root = state.selected;
    state.fallback_this_frame = false;
    state.clear_fallback_promotions();
}

pub fn pop_up(state: &mut AppState) {
    state.drawing_root = None;
    state.fallback_this_frame = false;
    state.clear_fallback_promotions();
}

pub fn toggle_triage(state: &mut AppState) {
    state.mode = "triage".into();
}

pub fn toggle_plugin(state: &mut AppState) {
    state.mode = "plugin".into();
}

pub fn toggle_keymap(state: &mut AppState) {
    state.show_keymap = !state.show_keymap;
}

pub fn toggle_settings(state: &mut AppState) {
    state.mode = "settings".into();
}

pub fn toggle_sticky_notes(state: &mut AppState) {
    state.toggle_sticky_overlay();
}

