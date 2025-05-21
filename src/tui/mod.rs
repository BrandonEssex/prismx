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
    use crate::render::*;
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

                // Handle hotkeys...
                // handlers module will be used here later

                if match_hotkey("quit", code, modifiers, &state) {
                    break;
                }

                // ... other hotkey checks

                // fallback spotlight + typing
                match code {
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
