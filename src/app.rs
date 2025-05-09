use crate::config::load_config;
use crate::screen::Screen;
use crate::state::AppState;
use crate::input::InputHandler;
use crate::actions::Action;
use crate::spotlight::SpotlightModule;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let spotlight = SpotlightModule::new();
    let mut state = AppState::new();
    let mut screen = Screen::new(config, spotlight);
    let input = InputHandler;

    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            screen.draw::<CrosstermBackend<_>>(f, &mut state);
        })?;

        if let Some(event) = input.poll_event()? {
            if let Some(action) = input.handle_event(event) {
                match action {
                    Action::Quit => state.quit(),
                    _ => screen.handle_action(action),
                }
            }
        }

        if !state.is_running() {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}