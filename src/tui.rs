use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Paragraph;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::stdout;
use crate::state::AppState;
use crate::render::*;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, state: &AppState, last_key: &str) -> std::io::Result<()> {
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
            _ => {
                let fallback = Paragraph::new("Unknown mode");
                f.render_widget(fallback, vertical[0]);
            }
        }

        if state.show_triage {
            render_triage(f, vertical[0]);
        }

        if state.show_spotlight {
            render_spotlight(f, vertical[0], &state.spotlight_input);
        }

        if state.show_keymap && layout_chunks.len() > 1 {
            render_keymap_overlay(f, layout_chunks[1]);
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

                match code {
                    KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => break,

                    KeyCode::Char('m') if modifiers.contains(KeyModifiers::CONTROL) => {
                        state.mode = "mindmap".into();
                    }

                    KeyCode::Char('z') if modifiers.contains(KeyModifiers::CONTROL) => {
                        state.mode = "zen".into();
                    }

                    KeyCode::Char('i') if modifiers.contains(KeyModifiers::CONTROL) => {
                        state.show_triage = !state.show_triage;
                    }

                    KeyCode::Char('h') if modifiers.contains(KeyModifiers::CONTROL) => {
                        state.show_keymap = !state.show_keymap;
                    }

                    KeyCode::Char('\u{a0}') | KeyCode::Char(' ') if modifiers.contains(KeyModifiers::ALT) => {
                        state.show_spotlight = !state.show_spotlight;
                    }

                    KeyCode::Char('7') if modifiers.contains(KeyModifiers::CONTROL) => {
                        state.show_spotlight = !state.show_spotlight;
                    }

                    // Spotlight input
                    KeyCode::Char(c) if modifiers.is_empty() && state.show_spotlight => {
                        state.spotlight_input.push(c);
                    }

                    KeyCode::Backspace if state.show_spotlight => {
                        state.spotlight_input.pop();
                    }

                    KeyCode::Enter if state.show_spotlight => {
                        state.execute_spotlight_command();
                    }

                    // Escape clears overlays
                    KeyCode::Esc => {
                        state.mode = "mindmap".into();
                        state.show_keymap = false;
                        state.show_spotlight = false;
                        state.show_triage = false;
                    }

                    // Navigation
                    KeyCode::Up if state.mode == "mindmap" && !state.show_spotlight => {
                        state.move_focus_up();
                    }

                    KeyCode::Down if state.mode == "mindmap" && !state.show_spotlight => {
                        state.move_focus_down();
                    }

                    // Toggle Edit Mode
                    KeyCode::Char('e') if modifiers.contains(KeyModifiers::CONTROL) && state.mode == "mindmap" => {
                        state.edit_mode = !state.edit_mode;
                    }

                    // Direct editing
                    KeyCode::Char(c) if state.mode == "mindmap" && state.edit_mode => {
                        state.update_active_label(c);
                    }

                    KeyCode::Backspace if state.mode == "mindmap" && state.edit_mode => {
                        state.delete_last_char();
                    }

                    // Add sibling / child
                    KeyCode::Enter if state.mode == "mindmap" && state.edit_mode => {
                        state.add_sibling();
                    }

                    KeyCode::Tab if state.mode == "mindmap" && state.edit_mode => {
                        state.add_child();
                    }

                    // Delete node
                    KeyCode::Delete | KeyCode::Backspace if modifiers.contains(KeyModifiers::SHIFT) && state.mode == "mindmap" && state.edit_mode => {
                        state.delete_node();
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
