use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
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

        let zones = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(50), Constraint::Length(30)].as_ref())
            .split(full);

        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(zones[0]);

        // Main mode rendering
        match state.mode.as_str() {
            "zen" => render_zen_journal(f, main[0], state),
            "mindmap" => render_mindmap(f, main[0], state),
            _ => {
                let fallback = Paragraph::new("Unknown mode");
                f.render_widget(fallback, main[0]);
            }
        }

        // Right-hand overlays
        if state.show_keymap {
            render_keymap_overlay(f, zones[1]);
        }

        // Overlay panels on top of main view
        if state.show_triage {
            render_triage(f, main[0]);
        }

        if state.show_spotlight {
            render_spotlight(f, main[0], &state.spotlight_input);
        }

        let status_text = format!(
            "Mode: {} | Triage: {} | Spotlight: {} | Help: {}",
            state.mode,
            state.show_triage,
            state.show_spotlight,
            state.show_keymap
        );
        render_status_bar(f, main[1], &status_text);
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
                    // Quit
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => break,

                    // Mode switching
                    KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.mode = "mindmap".into();
                    }

                    KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.mode = "zen".into();
                    }

                    // Triage (Ctrl+I = Tab)
                    KeyCode::Tab => {
                        state.show_triage = !state.show_triage;
                    }

                    // Help (Ctrl+H)
                    KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.show_keymap = !state.show_keymap;
                    }

                    // Spotlight (Alt+Space or Ctrl+/)
                    KeyCode::Char('\u{a0}') | KeyCode::Char(' ') if key.modifiers.contains(KeyModifiers::ALT) => {
                        state.show_spotlight = !state.show_spotlight;
                    }
                    KeyCode::Char('/') if key.modifiers.contains(KeyModifiers::CONTROL) => {
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

                    // Escape resets
                    KeyCode::Esc => {
                        state.mode = "mindmap".into();
                        state.show_keymap = false;
                        state.show_spotlight = false;
                        state.show_triage = false;
                    }

                    // Zen input
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
