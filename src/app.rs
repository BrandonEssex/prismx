use crate::{
    actions::Action,
    config::load_config,
    input::{InputHandler},
    screen::Screen,
    state::AppState,
};

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    let mut state = AppState::new();
    let mut screen = Screen::new();

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;

    let input = InputHandler;

    loop {
        terminal.draw(|f| {
            screen.draw(f, &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if let Some(action) = input.handle_event(Event::Key(key_event)) {
                    match action {
                        Action::Quit => break,
                        _ => screen.handle_action(action, &mut state),
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}