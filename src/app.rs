use crate::config::load_config;
use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use crate::screen::Screen;
use crate::extension_host::ExtensionHost;
use crate::input::InputHandler;
use crate::actions::Action;

use crossterm::terminal;
use ratatui::{Terminal};
use ratatui::backend::CrosstermBackend;
use std::io::stdout;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut state = AppState::new(config.clone());
    let spotlight = SpotlightModule::new();
    let mut screen = Screen::new(config, spotlight);

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut ext_host = ExtensionHost::new();
    ext_host.load_all()?;

    let input = InputHandler;

    while state.is_running() {
        terminal.draw(|f| {
            screen.draw(f, &mut state);
        })?;

        if let Some(event) = input.poll_event()? {
            if let Some(action) = input.handle_event(event) {
                match action {
                    Action::Quit => state.quit(),
                    _ => screen.handle_action(action),
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}