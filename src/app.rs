use crate::config::load_config;
use crate::input::InputHandler;
use crate::screen::Screen;
use crate::state::AppState;
use crate::extension_host::ExtensionHost;
use crate::spotlight::SpotlightModule;
use crate::actions::Action;

use std::io::stdout;
use crossterm::terminal;
use ratatui::{prelude::Backend, Frame};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut state = AppState::new(config.clone());
    let spotlight = SpotlightModule::new();
    let mut screen = Screen::new(config, spotlight);

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut ext_host = ExtensionHost::new();
    ext_host.load_all()?;

    let input = InputHandler;

    while state.is_running() {
        // FIXED: Temporarily log instead of calling draw() with unresolved backend
        log::info!("Drawing skipped (frame backend unimplemented)");

        if let Some(event) = input.poll_event()? {
            if let Some(action) = input.handle_event(event) {
                match action {
                    Action::Quit => state.quit(),
                    _ => {}
                }
            }
        }
    }

    crossterm::execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// Placeholder function remains, but unused
fn screen_terminal_frame<'a, B: Backend>() -> Frame<'a> {
    unimplemented!("This should return the real terminal frame for drawing.")
}