use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

use crate::state::AppState;
use crate::render::{
    render_status_bar,
    render_zen_journal,
    render_keymap_overlay,
    render_spotlight,
    render_triage,
    render_module_switcher,
};
use crate::screen::render_gemx;

mod hotkeys;
use hotkeys::match_hotkey;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, state: &mut AppState, last_key: &str) -> std::io::Result<()> {
    use ratatui::layout::{Constraint, Direction, Layout};
    use ratatui::widgets::{Block, Borders, Paragraph};

    terminal.draw(|f| {
        let full = f.size();
        let layout_chunks = if state.show_keymap {
            Layout::default().direction(Direction::Horizontal)
                .constraints([Constraint::Min(50), Constraint::Length(30)].as_ref())
                .split(full)
        } else {
            std::rc::Rc::from(vec![full])
        };

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(layout_chunks[0]);

        match state.mode.as_str() {
            "zen" => render_zen_journal(f, vertical[0], state),
            "gemx" => render_gemx(f, vertical[0], &state),
            "settings" => {
                let fallback = Paragraph::new("Settings panel coming soon...")
                    .block(Block::default().title("Settings").borders(Borders::ALL));
                f.render_widget(fallback, vertical[0]);
            }
            "triage" => render_triage(f, vertical[0]),
            _ => {
                let fallback = Paragraph::new("Unknown mode");
                f.render_widget(fallback, vertical[0]);
            }
        }

        if state.show_spotlight {
            render_spotlight(f, vertical[0], &state.spotlight_input);
        }

        if state.show_keymap && layout_chunks.len() > 1 {
            render_keymap_overlay(f, layout_chunks[1]);
        }

        if state.module_switcher_open {
            render_module_switcher(f, vertical[0], state.module_switcher_index);
        }

        let status_text = format!(
            "Mode: {} | Triage: {} | Spotlight: {} | Help: {} | Last Key: {}",
            state.mode,
            state.show_triage,
            state.show_spotlight,
            state.show_keymap,
            last_key
        );
        render_status_bar(f, vertical[1], &status_text);
    })?;
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
            state.selected = Some(state.nodes.keys().copied().next().unwrap());
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                last_key = format!("{:?} + {:?}", code, modifiers);

                // 🌟 Spotlight
                if state.show_spotlight {
                    match code {
                        KeyCode::Esc => state.show_spotlight = false,
                        KeyCode::Char(c) => state.spotlight_input.push(c),
                        KeyCode::Backspace => { state.spotlight_input.pop(); }
                        KeyCode::Enter => state.execute_spotlight_command(),
                        _ => {}
                    }
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // Alt+Space = spotlight
                if code == KeyCode::Char('\u{a0}') {
                    state.show_spotlight = !state.show_spotlight;
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // 🧭 Module switcher
                if state.module_switcher_open {
                    match code {
                        KeyCode::Tab => {
                            state.module_switcher_index = (state.module_switcher_index + 1) % 4;
                        }
                        KeyCode::Enter => {
                            state.mode = state.get_module_by_index().into();
                            state.module_switcher_open = false;
                        }
                        KeyCode::Esc => {
                            state.module_switcher_open = false;
                        }
                        _ => {}
                    }
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // 🎯 Hotkeys
                if match_hotkey("quit", code, modifiers, &state) {
                    break;
                } else if match_hotkey("toggle_triage", code, modifiers, &state) {
                    state.mode = "triage".into();
                } else if match_hotkey("toggle_keymap", code, modifiers, &state) {
                    state.show_keymap = !state.show_keymap;
                } else if match_hotkey("create_child", code, modifiers, &state) && state.mode == "gemx" {
                    state.push_undo(); state.add_child();
                } else if match_hotkey("create_sibling", code, modifiers, &state) && state.mode == "gemx" {
                    state.push_undo(); state.add_sibling();
                } else if match_hotkey("add_free_node", code, modifiers, &state) {
                    state.push_undo(); state.add_free_node();
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
                    state.module_switcher_index = 0;
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
                } else if match_hotkey("switch_workspace", code, modifiers, &state) {
                    state.switch_workspace();
                } else if match_hotkey("save_workspace", code, modifiers, &state) {
                    state.save_workspace();
                } else if match_hotkey("load_workspace", code, modifiers, &state) {
                    state.load_workspace();
                } else if match_hotkey("save", code, modifiers, &state) {
                    state.export_zen_to_file();
                } else if match_hotkey("mode_zen", code, modifiers, &state) {
                    state.mode = "zen".into();
                } else if match_hotkey("toggle_collapsed", code, modifiers, &state) && state.mode == "gemx" {
                    state.toggle_collapse();
                } else if match_hotkey("drill_down", code, modifiers, &state) {
                    state.drawing_root = state.selected;
                } else if match_hotkey("pop_up", code, modifiers, &state) {
                    state.drawing_root = None;
                } else if match_hotkey("toggle_settings", code, modifiers, &state) {
                    state.mode = "settings".into();
                }

                // 🍎 macOS fallback for Cmd+Arrow scrolling
                if crate::input::mac_fallback::handle_cmd_arrows(code, modifiers, &mut state) {
                    draw(&mut terminal, &mut state, &last_key)?;
                    continue;
                }

                // ⌨️ Navigation + Typing
                match code {
                    KeyCode::Esc => {
                        state.mode = "gemx".into();
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

                    KeyCode::Up if state.mode == "gemx" => state.move_focus_up(),
                    KeyCode::Down if state.mode == "gemx" => state.move_focus_down(),
                    KeyCode::Left if state.mode == "gemx" => state.move_focus_left(),
                    KeyCode::Right if state.mode == "gemx" => state.move_focus_right(),
                    KeyCode::Tab if state.mode == "gemx" => state.move_focus_right(),
                    KeyCode::BackTab if state.mode == "gemx" => state.move_focus_left(),

                    KeyCode::Char(c) if state.mode == "gemx" => {
                        let allowed = modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT;
                        if allowed && (c.is_ascii_graphic() || c == ' ') {
                            if let Some(node) = state.get_selected_node_mut() {
                                if node.label == "New Child"
                                    || node.label == "New Sibling"
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

                    KeyCode::Char(c) if state.mode == "zen" => {
                        state.push_undo();
                        if let Some(last) = state.zen_buffer.last_mut() {
                            last.push(c);
                        }
                    }

                    KeyCode::Backspace if state.mode == "zen" => {
                        if let Some(last) = state.zen_buffer.last_mut() {
                            last.pop();
                        }
                    }

                    KeyCode::Enter if state.mode == "zen" => {
                        state.zen_buffer.push(String::new());
                    }

                    _ => {}
                }
            }
        }

        draw(&mut terminal, &mut state, &last_key)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
