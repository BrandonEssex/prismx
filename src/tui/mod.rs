pub mod layout;
pub mod widgets;
pub mod events;

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::Stdout;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use crate::state::view::ZenLayoutMode;
use crate::state::{AppState, SimInput, ZenViewMode};
use crate::render::{
    render_status_bar,
    render_shortcuts_overlay,
    render_spotlight,
    render_triage,
    render_module_icon,
    Renderable,
    ZenView,
};

use layout::rect_contains;
use crate::screen::render_gemx;
use crate::render::render_settings;
use crate::settings::{SETTING_CATEGORIES, toggles_for_category};
use crate::ui::components::plugin::render_plugin;
use crate::ui::components::logs::render_logs;
use crate::ui::input;
use crate::modules::triage::input as triage_input;
use crate::modules::switcher;
use crate::modules::switcher::render::AppSwitcher;

use crate::hotkeys::match_hotkey;
use crate::shortcuts::{match_shortcut, Shortcut};

pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: &mut AppState,
    _last_key: &str,
) -> std::io::Result<()> {
    use ratatui::layout::{Constraint, Direction, Layout, Rect};

    if !state.auto_arrange {
        state.ensure_grid_positions();
    }

    state.animate_scroll();

    if state.show_spotlight && !state.prev_show_spotlight {
        state.spotlight_just_opened = true;
        state.spotlight_animation_frame = 0;
    }

    if state.show_keymap && !state.prev_show_keymap {
        state.keymap_animation_frame = 0;
    }
    if !state.show_keymap && state.prev_show_keymap {
        state.keymap_closing = true;
        state.keymap_animation_frame = 0;
    }

    if state.module_switcher_open && !state.prev_module_switcher_open {
        state.module_switcher_animation_frame = 0;
    }
    if !state.module_switcher_open && state.prev_module_switcher_open {
        state.module_switcher_closing = true;
        state.module_switcher_animation_frame = 0;
    }
    if state.mode != state.prev_mode {
        if state.mode == "gemx" {
            state.mindmap_title_frames = 6;
        }
    }

    terminal.draw(|f| {
        let full = f.size();
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(full);

        let mut views: Vec<Box<dyn Renderable>> = Vec::new();
        match state.mode.as_str() {
            "zen" => views.push(Box::new(ZenView::new(state))),
            "gemx" => render_gemx(f, vertical[0], state),
            "settings" => render_settings(f, vertical[0], state),
            "triage" => render_triage(f, vertical[0], state),
            "plugin" => render_plugin(f, vertical[0], state),
            _ => {
                widgets::render_unknown_mode(f, vertical[0]);
            }
        }

        if state.module_switcher_open || state.module_switcher_closing {
            views.push(Box::new(AppSwitcher::new(state)));
        }

        for mut view in views {
            view.render(f, vertical[0]);
        }

        if state.show_keymap || state.keymap_closing {
            render_shortcuts_overlay(f, vertical[0], state);
        }

        if state.show_spotlight {
            render_spotlight(f, vertical[0], state);
        }

        // status bar is rendered separately based on AppState
        render_module_icon(f, full, &state.mode);
        if state.show_logs {
            render_logs(f, full, state);
        }

        if state.debug_overlay || state.debug_overlay_sticky {
            let width = 30;
            let height = 9;
            let x = full.right().saturating_sub(width + 1);
            let area = Rect::new(x, full.y, width, height);
            crate::ui::overlay::render_debug_overlay(
                f,
                area,
                state,
                state.debug_overlay_sticky,
            );
        }

        crate::ui::components::debug::render_debug(f, full, state);

        render_status_bar(f, vertical[1], state);
    })?;
    if state.spotlight_just_opened {
        state.spotlight_animation_frame += 1;
        if state.spotlight_animation_frame >= 3 {
            state.spotlight_just_opened = false;
        }
    }
    if state.show_keymap && state.keymap_animation_frame < 3 {
        state.keymap_animation_frame += 1;
    } else if state.keymap_closing {
        state.keymap_animation_frame += 1;
        if state.keymap_animation_frame >= 3 {
            state.keymap_closing = false;
        }
    }
    if state.module_switcher_open && state.module_switcher_animation_frame < 3 {
        state.module_switcher_animation_frame += 1;
    } else if state.module_switcher_closing {
        state.module_switcher_animation_frame += 1;
        if state.module_switcher_animation_frame >= 3 {
            state.module_switcher_closing = false;
        }
    }
    state.prev_module_switcher_open = state.module_switcher_open;
    state.prev_show_spotlight = state.show_spotlight;
    state.tick_journal_entry_frames();
    state.prev_show_keymap = state.show_keymap;
    state.prev_mode = state.mode.clone();
    Ok(())
}

pub fn launch_ui() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::default();
    let mut last_key = String::new();

    draw(&mut terminal, &mut state, &last_key)?;

    loop {
        if state.selected.is_none() && !state.nodes.is_empty() {
            let first = state.nodes.keys().copied().next().unwrap();
            state.set_selected(Some(first));
        }

        state.auto_save_zen();

        if let Some(sim_input) = state.simulate_input_queue.pop_front() {
            match sim_input {
                SimInput::Enter => state.add_sibling_node(),
                SimInput::Tab => state.add_child_node(),
                SimInput::Delete => state.delete_node(),
                SimInput::Drill => state.drill_selected(),
                SimInput::Pop => state.pop_stack(),
                SimInput::Undo => state.undo(),
                SimInput::Redo => state.redo(),
            }

            if state.debug_input_mode {
                tracing::debug!("\u{1F9EA} Simulated input: {:?}", sim_input);
                if state.simulate_input_queue.is_empty() {
                    tracing::debug!("\u{1F9EA} Simulation complete.");
                }
            }
        }

        if let Some(event) = events::next_event(std::time::Duration::from_millis(100))? {
            match event {
                Event::Key(KeyEvent { code, modifiers, .. }) => {
                    last_key = format!("{:?} + {:?}", code, modifiers);
                    if state.debug_input_mode {
                        crate::hotkeys::debug_input(&mut state, code, modifiers);
                    }
                    if state.show_logs {
                        if input::handle_log_keys(&mut state, code, modifiers) {
                            draw(&mut terminal, &mut state, &last_key)?;
                            continue;
                        }
                    }
                    if let Some(sc) = match_shortcut(code, modifiers) {
                        match sc {
                            Shortcut::ToggleDebugInput => {
                                state.debug_input_mode = !state.debug_input_mode;
                            }
                            Shortcut::ZoomIn => state.zoom_in(),
                            Shortcut::ZoomOut => state.zoom_out(),
                            Shortcut::ToggleDebugBorder => {
                                state.toggle_debug_border();
                            }
                        }
                    }

                // 🌟 Spotlight
                if state.show_spotlight {
                    match code {
                        KeyCode::Esc => {
                            state.show_spotlight = false;
                            state.spotlight_history_index = None;
                        }
                        KeyCode::Char(c) => {
                            state.spotlight_input.push(c);
                            state.spotlight_history_index = None;
                            state.spotlight_suggestion_index = None;
                        }
                        KeyCode::Backspace => {
                            state.spotlight_input.pop();
                            state.spotlight_history_index = None;
                            state.spotlight_suggestion_index = None;
                        }
                        KeyCode::Up => {
                            let suggestions = crate::spotlight::command_suggestions(&state.spotlight_input);
                            if !suggestions.is_empty() {
                                let len = suggestions.len();
                                let idx = state.spotlight_suggestion_index.unwrap_or(0);
                                state.spotlight_suggestion_index = Some((idx + len - 1) % len);
                            } else if state.spotlight_history.is_empty() {
                                // nothing
                            } else if let Some(i) = state.spotlight_history_index {
                                if i + 1 < state.spotlight_history.len() {
                                    state.spotlight_history_index = Some(i + 1);
                                }
                                if let Some(idx) = state.spotlight_history_index {
                                    state.spotlight_input = state.spotlight_history[idx].clone();
                                }
                            } else {
                                state.spotlight_history_index = Some(0);
                                state.spotlight_input = state.spotlight_history[0].clone();
                            }
                        }
                        KeyCode::Down => {
                            let suggestions = crate::spotlight::command_suggestions(&state.spotlight_input);
                            if !suggestions.is_empty() {
                                let len = suggestions.len();
                                let idx = state.spotlight_suggestion_index.unwrap_or(len);
                                state.spotlight_suggestion_index = Some((idx + 1) % len);
                            } else if let Some(i) = state.spotlight_history_index {
                                if i > 0 {
                                    state.spotlight_history_index = Some(i - 1);
                                    if let Some(idx) = state.spotlight_history_index {
                                        state.spotlight_input = state.spotlight_history[idx].clone();
                                    }
                                } else {
                                    state.spotlight_history_index = None;
                                    state.spotlight_input.clear();
                                }
                            }
                        }
                        KeyCode::Right | KeyCode::Tab => {
                            let suggestions = crate::spotlight::command_suggestions(&state.spotlight_input);
                            if let Some(sel) = state.spotlight_suggestion_index {
                                if let Some(s) = suggestions.get(sel) {
                                    state.spotlight_input = s.to_string();
                                }
                                state.spotlight_suggestion_index = None;
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(sel) = state.spotlight_suggestion_index {
                                let suggestions = crate::spotlight::command_suggestions(&state.spotlight_input);
                                if let Some(s) = suggestions.get(sel) {
                                    state.spotlight_input = s.to_string();
                                }
                            }
                            state.execute_spotlight_command();
                        }
                        _ => {}
                    }

                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // Alt+Space or Alt+/ toggles Spotlight
                if (code == KeyCode::Char(' ') || code == KeyCode::Char('/'))
                    && modifiers == KeyModifiers::ALT
                {
                    state.show_spotlight = !state.show_spotlight;
                    tracing::info!(
                        "[INPUT] spotlight {}",
                        if state.show_spotlight { "opened" } else { "closed" }
                    );
                    state.spotlight_history_index = None;
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // Alt+L toggles log viewer
                if code == KeyCode::Char('l') && modifiers == KeyModifiers::ALT {
                    state.show_logs = !state.show_logs;
                    if !state.show_logs {
                        state.logs_filter.clear();
                        state.logs_scroll = 0;
                    }
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // 🧭 Module switcher
                if switcher::input::handle_key(&mut state, code, modifiers) {
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                if state.zen_toolbar_open {
                    state.zen_toolbar_handle_key(code);
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // 🎯 Hotkeys
                if match_hotkey("quit", code, modifiers, &state) {
                    break;
                } else if match_hotkey("toggle_triage", code, modifiers, &state) {
                    state.mode = "triage".into();
                    tracing::info!("[INPUT] mode switched to triage");
                } else if match_hotkey("toggle_plugin", code, modifiers, &state) {
                    state.mode = "plugin".into();
                    tracing::info!("[INPUT] mode switched to plugin");
                } else if match_hotkey("toggle_keymap", code, modifiers, &state) {
                    state.show_keymap = !state.show_keymap;
                } else if match_hotkey("create_child", code, modifiers, &state) && state.mode == "gemx" {
                    state.push_undo();
                    state.handle_tab_key();
                    continue;
                } else if match_hotkey("create_sibling", code, modifiers, &state) && state.mode == "gemx" {
                    state.push_undo();
                    state.handle_enter_key();
                    continue;
                } else if match_hotkey("add_free_node", code, modifiers, &state) {
                    state.push_undo();
                    crate::gemx::interaction::spawn_free_node(&mut state);
                } else if match_hotkey("create_branch", code, modifiers, &state) {
                    state.push_undo(); /* placeholder for create_branch() */
                } else if match_hotkey("delete", code, modifiers, &state) && state.mode == "gemx" {
                    state.push_undo(); state.delete_node();
                } else if match_hotkey("undo", code, modifiers, &state) {
                    state.undo();
                } else if match_hotkey("redo", code, modifiers, &state) {
                    state.redo();
                } else if match_hotkey("open_module_switcher", code, modifiers, &state) {
                    state.module_switcher_open = true;
                    state.module_switcher_index = switcher::index_for_mode(&state.mode);
                    tracing::info!("[INPUT] module switcher opened");
                } else if match_hotkey("start_drag", code, modifiers, &state) {
                    if state.selected_drag_source.is_some() {
                        if let Some(target) = state.selected {
                            state.push_undo();
                            state.complete_drag(target);
                        }
                    } else {
                        state.start_drag();
                    }
                } else if match_hotkey("start_link", code, modifiers, &state) {
                    if state.selected_drag_source.is_some() {
                        if let Some(target) = state.selected {
                            state.complete_link(target);
                        }
                    } else {
                        state.start_link();
                    }
                } else if match_hotkey("toggle_link_mode", code, modifiers, &state) {
                    state.link_mode = !state.link_mode;
                    crate::log_debug!(state, "link_mode toggled: {}", state.link_mode);
                } else if match_hotkey("save", code, modifiers, &state) {
                    state.export_zen_to_file();
                } else if match_hotkey("mode_zen", code, modifiers, &state) {
                    state.mode = "zen".into();
                    tracing::info!("[INPUT] mode switched to zen");
                } else if match_hotkey("zen_toggle_theme", code, modifiers, &state) && state.mode == "zen" {
                    state.cycle_zen_theme();
                } else if match_hotkey("debug_snapshot", code, modifiers, &state) {
                    crate::ui::components::debug::write_debug_snapshot(&mut state);
                } else if match_hotkey("debug_overlay", code, modifiers, &state) {
                    state.debug_overlay = !state.debug_overlay;
                } else if match_hotkey("debug_overlay_sticky", code, modifiers, &state) {
                    state.debug_overlay_sticky = !state.debug_overlay_sticky;
                    state.debug_overlay = state.debug_overlay_sticky;
                } else if match_hotkey("reload_plugins", code, modifiers, &state) {
                    crate::state::init::reload_plugins();
                } else if code == KeyCode::Char('v')
                    && modifiers.contains(KeyModifiers::CONTROL)
                    && modifiers.contains(KeyModifiers::SHIFT)
                    && state.mode == "zen"
                {
                    state.zen_layout_mode = match state.zen_layout_mode {
                        ZenLayoutMode::Journal => ZenLayoutMode::Classic,
                        ZenLayoutMode::Classic => ZenLayoutMode::Split,
                        ZenLayoutMode::Split => ZenLayoutMode::Summary,
                        ZenLayoutMode::Summary => ZenLayoutMode::Dual,
                        ZenLayoutMode::Dual => ZenLayoutMode::Journal,
                        ZenLayoutMode::Compose => ZenLayoutMode::Journal,
                    };
                } else if code == KeyCode::Char('t')
                    && modifiers == KeyModifiers::CONTROL
                    && state.mode == "zen"
                {
                    state.zen_toolbar_open = !state.zen_toolbar_open;
                    state.zen_toolbar_index = 0;
                } else if code == KeyCode::Tab
                    && modifiers == KeyModifiers::CONTROL
                    && state.mode == "zen"
                {
                    input::toggle_zen_view(&mut state);
                } else if match_hotkey("toggle_collapsed", code, modifiers, &state) && state.mode == "gemx" {
                    state.toggle_collapse();
                } else if match_hotkey("drill_down", code, modifiers, &state) {
                    state.drawing_root = state.selected;
                    state.fallback_this_frame = false;
                    state.clear_fallback_promotions();
                } else if match_hotkey("pop_up", code, modifiers, &state) {
                    state.drawing_root = None;
                    state.fallback_this_frame = false;
                    state.clear_fallback_promotions();
                } else if match_hotkey("toggle_settings", code, modifiers, &state) {
                    state.mode = "settings".into();
                    tracing::info!("[INPUT] mode switched to settings");
                } else if match_hotkey("toggle_sticky_notes", code, modifiers, &state) && state.mode == "triage" {
                    state.toggle_sticky_overlay();
                } else if code == KeyCode::Char('.') && modifiers == KeyModifiers::CONTROL {
                    state.mode = "settings".into();
                    tracing::info!("[INPUT] mode switched to settings");
                }

                // 🍎 macOS fallback for Cmd+Arrow scrolling
                if crate::input::mac_fallback::handle_cmd_arrows(code, modifiers, &mut state) {
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                if modifiers.contains(KeyModifiers::CONTROL) {
                    if let KeyCode::Char(digit) = code {
                        if ('1'..='5').contains(&digit) {
                            let idx = digit as usize - '1' as usize;
                            state.trigger_favorite(idx);
                            draw(&mut terminal, &mut state, &last_key)?;
                            continue;
                        }
                    }
                }

                // ⌨️ Navigation + Typing
                match code {
                    KeyCode::Esc => {
                        if state.mode == "zen"
                            && state.zen_layout_mode == ZenLayoutMode::Compose
                            && state.zen_view_mode == ZenViewMode::Write
                            && state.zen_draft.editing.is_some()
                        {
                            state.cancel_edit_journal_entry();
                            state.zen_draft.text.clear();
                        } else if state.mode == "plugin" && state.show_plugin_preview {
                            state.show_plugin_preview = false;
                        } else if state.mode == "triage" && state.triage_tag_filter.is_some() {
                            state.triage_set_filter(None);
                        } else {
                            state.mode = "gemx".into();
                            tracing::info!("[INPUT] mode switched to gemx");
                        }
                        state.show_keymap = false;
                        state.show_spotlight = false;
                    }


                    KeyCode::Left if state.mode == "gemx" && modifiers == KeyModifiers::CONTROL => {
                        state.scroll_x = state.scroll_x.saturating_sub(4);
                    }

                    KeyCode::Right if state.mode == "gemx" && modifiers == KeyModifiers::CONTROL => {
                        state.scroll_x = state.scroll_x.saturating_add(4);
                    }

                    KeyCode::Char('p') if modifiers == KeyModifiers::CONTROL && state.mode == "gemx" => {
                        state.auto_arrange = !state.auto_arrange;
                    }

                    KeyCode::Char('g') if modifiers == KeyModifiers::CONTROL && state.mode == "gemx" => {
                        state.toggle_snap_grid();
                    }

                    KeyCode::Left if state.mode == "settings" => {
                        let total_tabs = crate::settings::SETTING_CATEGORIES.len() + state.plugin_tabs.len();
                        if state.settings_selected_tab == 0 {
                            state.settings_selected_tab = total_tabs.saturating_sub(1);
                        } else {
                            state.settings_selected_tab -= 1;
                        }
                        state.settings_focus_index = 0;
                    }
                    KeyCode::Right if state.mode == "settings" => {
                        let total_tabs = crate::settings::SETTING_CATEGORIES.len() + state.plugin_tabs.len();
                        state.settings_selected_tab = (state.settings_selected_tab + 1) % total_tabs;
                        state.settings_focus_index = 0;
                    }
                    KeyCode::Up if state.mode == "settings" && state.settings_selected_tab < crate::settings::SETTING_CATEGORIES.len() => {
                        let toggles = crate::settings::toggles_for_category(crate::settings::SETTING_CATEGORIES[state.settings_selected_tab]);
                        if !toggles.is_empty() {
                            if state.settings_focus_index == 0 {
                                state.settings_focus_index = toggles.len() - 1;
                            } else {
                                state.settings_focus_index -= 1;
                            }
                        }
                    }
                    KeyCode::Down if state.mode == "settings" && state.settings_selected_tab < crate::settings::SETTING_CATEGORIES.len() => {
                        let toggles = crate::settings::toggles_for_category(crate::settings::SETTING_CATEGORIES[state.settings_selected_tab]);
                        if !toggles.is_empty() {
                            state.settings_focus_index = (state.settings_focus_index + 1) % toggles.len();
                        }
                    }
                    KeyCode::Enter | KeyCode::Char(' ') if state.mode == "settings" && state.settings_selected_tab < crate::settings::SETTING_CATEGORIES.len() => {
                        let toggles = crate::settings::toggles_for_category(crate::settings::SETTING_CATEGORIES[state.settings_selected_tab]);
                        if !toggles.is_empty() {
                            let global_idx = toggles[state.settings_focus_index % toggles.len()].0;
                            (crate::settings::SETTING_TOGGLES[global_idx].toggle)(&mut state);
                        }
                    }

                    KeyCode::Up if state.mode == "plugin" && !state.show_plugin_preview => {
                        let len = crate::plugin::registry::registry_filtered(state.plugin_tag_filter).len();
                        if len > 0 {
                            if state.plugin_registry_index == 0 {
                                state.plugin_registry_index = len - 1;
                            } else {
                                state.plugin_registry_index -= 1;
                            }
                        }
                    }
                    KeyCode::Down if state.mode == "plugin" && !state.show_plugin_preview => {
                        let len = crate::plugin::registry::registry_filtered(state.plugin_tag_filter).len();
                        if len > 0 {
                            state.plugin_registry_index = (state.plugin_registry_index + 1) % len;
                        }
                    }
                    KeyCode::Enter if state.mode == "plugin" && !state.show_plugin_preview => {
                        state.show_plugin_preview = true;
                    }
                    KeyCode::Tab if state.mode == "plugin" && !state.show_plugin_preview => {
                        state.plugin_tag_filter = state.plugin_tag_filter.next();
                        state.plugin_registry_index = 0;
                    }

                    // --- Triage navigation ---
                    KeyCode::Up if state.mode == "triage" => {
                        state.triage_focus_prev();
                        state.triage_recalc_counts();
                    }
                    KeyCode::Down if state.mode == "triage" => {
                        state.triage_focus_next();
                        state.triage_recalc_counts();
                    }
                    KeyCode::Enter if state.mode == "triage" => {
                        state.triage_view_mode = match state.triage_view_mode {
                            crate::state::TriageViewMode::Feed => crate::state::TriageViewMode::Actions,
                            crate::state::TriageViewMode::Actions => crate::state::TriageViewMode::Feed,
                        };
                    }
                    KeyCode::Char('d') if state.mode == "triage" && modifiers == KeyModifiers::CONTROL => {
                        state.triage_delete_current();
                        state.triage_recalc_counts();
                    }
                    KeyCode::Char('1') if state.mode == "triage" && modifiers == KeyModifiers::CONTROL => {
                        state.triage_toggle_tag("#now");
                        state.triage_recalc_counts();
                        state.triage_set_filter(Some("#now"));
                    }
                    KeyCode::Char('2') if state.mode == "triage" && modifiers == KeyModifiers::CONTROL => {
                        state.triage_toggle_tag("#triton");
                        state.triage_recalc_counts();
                        state.triage_set_filter(Some("#triton"));
                    }
                    KeyCode::Char('3') if state.mode == "triage" && modifiers == KeyModifiers::CONTROL => {
                        state.triage_toggle_tag("#done");
                        state.triage_recalc_counts();
                        state.triage_set_filter(Some("#done"));
                    }



                    KeyCode::Up if state.mode == "gemx" => state.move_focus_up(),
                    KeyCode::Down if state.mode == "gemx" => state.move_focus_down(),
                    KeyCode::Left if state.mode == "gemx" => state.focus_prev_sibling(),
                    KeyCode::Right if state.mode == "gemx" => state.focus_next_sibling(),
                    KeyCode::Tab if state.mode == "gemx" => state.focus_next_sibling(),
                    KeyCode::BackTab if state.mode == "gemx" => state.handle_shift_tab_key(),

                    KeyCode::Char(c) if state.mode == "gemx" => {
                        let allowed = modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT;
                        if allowed && (c.is_ascii_graphic() || c == ' ') {
                            if let Some(node) = state.get_selected_node_mut() {
                                if node.label.starts_with("node ")
                                    || node.label == "Node A"
                                    || node.label == "Node B"
                                {
                                    node.label.clear();
                                }
                                node.label.push(c);
                            }
                        }
                    }

                    KeyCode::Backspace if state.mode == "gemx" => {
                        if let Some(node) = state.get_selected_node_mut() {
                            node.label.pop();
                        }
                    }

                    k @ _ if state.mode == "triage" && triage_input::handle_key(&mut state, k, modifiers) => {}
                    k @ _ if state.mode == "zen" => {
                        input::route_zen_keys(&mut state, k, modifiers);
                    }
                    _ => {}
                }
            }
                Event::Mouse(me) => {
                    use crossterm::event::{MouseButton, MouseEventKind};
                    match me.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            state.last_mouse_click = Some((me.column, me.row));
                            let mut handled = false;
                            for (idx, (rect, _cmd)) in state.dock_entry_bounds.iter().enumerate() {
                                if rect_contains(*rect, me.column, me.row) {
                                    state.trigger_favorite(idx);
                                    handled = true;
                                    break;
                                }
                            }
                            if !handled && state.mode == "settings" && state.settings_selected_tab < crate::settings::SETTING_CATEGORIES.len() {
                                for (rect, idx) in state.settings_toggle_bounds.iter() {
                                    if rect_contains(*rect, me.column, me.row) {
                                        state.settings_focus_index = *idx;
                                        let toggles = crate::settings::toggles_for_category(crate::settings::SETTING_CATEGORIES[state.settings_selected_tab]);
                                        if !toggles.is_empty() {
                                            let global_idx = toggles[*idx].0;
                                            (crate::settings::SETTING_TOGGLES[global_idx].toggle)(&mut state);
                                        }
                                        handled = true;
                                        break;
                                    }
                                }
                            }
                            if !handled && state.mode == "gemx" {
                                if let Some(id) = crate::gemx::interaction::node_at_position(&state, me.column, me.row) {
                                    crate::gemx::interaction::start_drag(&mut state, id, me.column, me.row);
                                    handled = true;
                                }
                            }
                            if !handled && state.mode == "triage" {
                                if triage_input::handle_mouse(&mut state, me) { handled = true; }
                            }
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            if state.mode == "gemx" {
                                crate::gemx::interaction::drag_update(&mut state, me.column, me.row);
                            }
                            if state.mode == "triage" {
                                triage_input::handle_mouse(&mut state, me);
                            }
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            if state.mode == "gemx" {
                                crate::gemx::interaction::end_drag(&mut state);
                            }
                            if state.mode == "triage" {
                                triage_input::handle_mouse(&mut state, me);
                            }
                        }
                        MouseEventKind::Moved => {
                            let mut hover = None;
                            for (idx, (rect, _)) in state.dock_entry_bounds.iter().enumerate() {
                                if rect_contains(*rect, me.column, me.row) {
                                    hover = Some(idx);
                                    break;
                                }
                            }
                            state.dock_hover_index = hover;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        draw(&mut terminal, &mut state, &last_key)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
