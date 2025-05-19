use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Direction, Layout};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::{stdout, Write};
use crate::state::AppState;
use crate::render::*;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, state: &AppState) -> std::io::Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(f.size());

        render_status_bar(f, chunks[0]);

        match state.mode.as_str() {
            "zen" => render_zen_journal(f, chunks[1], state),
            "mindmap" => render_mindmap(f, chunks[1], state),
            _ => render_status_bar(f, chunks[1]),
        }

        if state.show_keymap {
            render_keymap_overlay(f, chunks[1]);
        }

        if state.show_spotlight {
            render_spotlight(f, chunks[1], &state.spotlight_input);
        }

        if state.show_triage {
            let area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(10)])
                .split(chunks[1])[1];
            render_triage(f, area);
        }
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
    draw(&mut terminal, &state)?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    // Exit on Ctrl+Q
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,

                    // Toggles
                    (KeyCode::Char('i'), KeyModifiers::CONTROL) => {
                        state.show_triage = !state.show_triage;
                    }
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        state.show_keymap = !state.show_keymap;
                    }
                    (KeyCode::Char(' '), KeyModifiers::ALT) => {
                        state.show_spotlight = !state.show_spotlight;
                    }

                    // Spotlight input
                    (KeyCode::Char(c), KeyModifiers::NONE) if state.show_spotlight => {
                        state.spotlight_input.push(c);
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) if state.show_spotlight => {
                        state.spotlight_input.pop();
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) if state.show_spotlight => {
                        state.execute_spotlight_command();
                    }

                    // Zen mode input
                    (KeyCode::Char(c), KeyModifiers::NONE) if state.mode == "zen" && !state.show_spotlight => {
                        if let Some(last) = state.zen_buffer.last_mut() {
                            last.push(c);
                        }
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) if state.mode == "zen" && !state.show_spotlight => {
                        state.zen_buffer.push(String::new());
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) if state.mode == "zen" && !state.show_spotlight => {
                        if let Some(last) = state.zen_buffer.last_mut() {
                            last.pop();
                        }
                    }

                    _ => {}
                }
            }
        }

        draw(&mut terminal, &state)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
