use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Paragraph;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::stdout;
use crate::state::AppState;
use crate::render::*;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, state: &AppState) -> std::io::Result<()> {
    terminal.draw(|f| {
        let full = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(full);

        match state.mode.as_str() {
            "zen" => render_zen_journal(f, chunks[0], state),
            "mindmap" => render_mindmap(f, chunks[0], state),
            _ => {
                let fallback = Paragraph::new("Unknown mode");
                f.render_widget(fallback, chunks[0]);
            }
        }

        if state.show_triage {
            render_triage(f, chunks[0]);
        }

        if state.show_keymap {
            render_keymap_overlay(f, chunks[0]);
        }

        if state.show_spotlight {
            render_spotlight(f, chunks[0], &state.spotlight_input);
        }

        render_status_bar(f, chunks[1]);
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
                match key.code {
                    // Exit
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => break,

                    // Triage: Ctrl+I may appear as Tab
                    KeyCode::Tab => {
                        state.show_triage = !state.show_triage;
                    }

                    // Keymap toggle
                    KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.show_keymap = !state.show_keymap;
                    }

                    // Spotlight (Alt+Space, macOS workaround)
                    KeyCode::Char('\u{a0}') | KeyCode::Char(' ') if key.modifiers.contains(KeyModifiers::ALT) => {
                        state.show_spotlight = !state.show_spotlight;
                    }

                    // Spotlight input
                    KeyCode::Char(c) if key.modifiers.is_empty() && state.show_spotlight => {
                        state.spotlight_input.push(c);
                    }

                    KeyCode::Backspace if state.show_spotlight => {
                        state.spotlight_input.pop();
                    }

                    KeyCode::Enter if state.show_spotlight => {
                        state.execute_spotlight_command();
                    }

                    // Zen typing
                    KeyCode::Char(c) if key.modifiers.is_empty() && state.mode == "zen" && !state.show_spotlight => {
                        if let Some(last) = state.zen_buffer.last_mut() {
                            last.push(c);
                        }
                    }

                    KeyCode::Enter if state.mode == "zen" && !state.show_spotlight => {
                        state.zen_buffer.push(String::new());
                    }

                    KeyCode::Backspace if state.mode == "zen" && !state.show_spotlight => {
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
