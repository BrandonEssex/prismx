use crate::{
    input::{InputHandler},
    screen::Screen,
    config::load_config,
    state::AppState,
};
use crossterm::terminal;
use std::io::{stdout, Write};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    logger::init_logger(&config);

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let backend = ratatui::backend::CrosstermBackend::new(&mut stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let mut screen = Screen::new(config.clone());
    let mut state = AppState::new();

    let input = InputHandler;

    while state.running {
        terminal.draw(|f| {
            screen.draw(f, &mut state);
        })?;

        if let Some(event) = input.poll_event()? {
            if let Some(action) = input.handle_event(event) {
                screen.handle_action(action, &mut state);
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}