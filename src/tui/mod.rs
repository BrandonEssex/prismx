use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::stdout;

use crate::state::AppState;
use crate::render::*;

mod hotkeys;
use hotkeys::match_hotkey;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, state: &AppState, last_key: &str) -> std::io::Result<()> {
    use ratatui::layout::{Constraint, Direction, Layout};
    use ratatui::widgets::{Block, Borders, Paragraph};

    terminal.draw(|f| {
        let full = f.size();

        let layout_chunks = if state.show_keymap {
            Layout::default()
                .direction(Direction::Horizontal)
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
            "mindmap" => render_mindmap(f, vertical[0], state),
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

    draw(&mut terminal, &state, &last_key)?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                last_key = format!("{:?} + {:?}", code, modifiers);

                if match_hotkey("quit", code, modifiers, &state) {
                    break;
                } else if match_hotkey("toggle_edit", code, modifiers, &state) && state.mode == "mindmap" {
                    state.edit_mode = !state.edit_mode;
                    state.edit_ready = state.edit_mode;
                } else if match_hotkey("toggle_triage", code, modifiers, &state) {
                    state.mode = "triage".into();
                } else if match_hotkey("toggle_keymap", code, modifiers, &state) {
                    state.show_keymap = !state.show_keymap;
                } else if match_hotkey("create_child", code, modifiers, &state) && state.mode == "mindmap" && state.edit_mode {
                    state.add_child();
                } else if match_hotkey("create_sibling", code, modifiers, &state) && state.mode == "mindmap" && state.edit_mode {
                    state.add_sibling();
                } else if match_hotkey("add_free_node", code, modifiers, &state) {
                    state.add_free_node();
                } else if match_hotkey("delete", code, modifiers, &state) && state.mode == "mindmap" && state.edit_mode {
                    state.delete_node();
                } else if match_hotkey("save", code, modifiers, &state) && state.mode == "zen" {
                    state.export_zen_to_file();
                } else if match_hotkey("mode_zen", code, modifiers, &state) {
                    state.mode = "zen".into();
                } else if match_hotkey("drill_down", code, modifiers, &state) {
                    state.drill_down();
                } else if match_hotkey("switch_module", code, modifiers, &state) {
                    state.module_switcher_open = true;
                    state.module_switcher_index = 0;
                } else if match_hotkey("toggle_collapsed", code, modifiers, &state) && state.mode == "mindmap" {
                    let collapsed = {
                        let node_ref = state.get_active_node();
                        let is_collapsed = node_ref.borrow().collapsed;
                        is_collapsed
                    };
                    if collapsed {
                        state.expand_active_node();
                    } else {
                        state.collapse_active_node();
                    }
                }

                match code {
                    KeyCode::Esc => {
                        if state.module_switcher_open {
                            state.module_switcher_open = false;
                        } else if state.edit_mode {
                            state.edit_mode = false;
                        } else {
                            state.mode = "mindmap".into();
                            state.show_keymap = false;
                            state.show_spotlight = false;
                        }
                    }

                    KeyCode::Tab if state.module_switcher_open => {
                        state.module_switcher_index = (state.module_switcher_index + 1) % 4;
                    }

                    KeyCode::Enter if state.module_switcher_open => {
                        state.mode = state.get_module_by_index().into();
                        state.module_switcher_open = false;
                    }

                    KeyCode::Up if state.mode == "mindmap" && !state.show_spotlight => {
                        state.move_focus_up();
                    }

                    KeyCode::Down if state.mode == "mindmap" && !state.show_spotlight => {
                        state.move_focus_down();
                    }

                    KeyCode::Char(c) if state.mode == "mindmap" && state.edit_mode => {
                        let node = state.get_active_node();
                        let mut n = node.borrow_mut();
                        if state.edit_ready && (
                            n.label == "New Child" || n.label == "New Sibling" ||
                            n.label == "Node A" || n.label == "Node B"
                        ) {
                            n.label.clear();
                            state.edit_ready = false;
                        }
                        n.label.push(c);
                    }

                    KeyCode::Backspace if state.mode == "mindmap" && state.edit_mode => {
                        let node = state.get_active_node();
                        node.borrow_mut().label.pop();
                    }

                    KeyCode::Char(c) if state.mode == "zen" => {
                        if let Some(last) = state.zen_buffer.last_mut() {
                            last.push(c);
                        }
                    }

                    KeyCode::Enter if state.mode == "zen" => {
                        state.zen_buffer.push(String::new());
                    }

                    KeyCode::Backspace if state.mode == "zen" => {
                        if let Some(last) = state.zen_buffer.last_mut() {
                            last.pop();
                        }
                    }

                    KeyCode::Char('\u{a0}') => {
                        state.show_spotlight = !state.show_spotlight;
                    }

                    KeyCode::Char(c) if state.show_spotlight => {
                        state.spotlight_input.push(c);
                    }

                    KeyCode::Backspace if state.show_spotlight => {
                        state.spotlight_input.pop();
                    }

                    KeyCode::Enter if state.show_spotlight => {
                        state.execute_spotlight_command();
                    }

                    _ => {}
                }
            }
        }

        draw(&mut terminal, &state, &last_key)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
