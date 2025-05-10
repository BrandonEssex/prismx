use crate::{
    config::Config,
    input::{InputHandler, KM},
    screen::Screen,
    state::AppState,
};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{stdout, Write};

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::new(config.clone());
    let spotlight = crate::spotlight::SpotlightModule::new();
    let mut screen = Screen::new(config.clone(), spotlight);

    enable_raw_mode()?;
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let input = InputHandler;

    while state.is_running() {
        terminal.draw(|f| {
            screen.draw(f, &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let Some(action) = input.handle_event(key) {
                    screen.handle_action(action, &mut state);
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}