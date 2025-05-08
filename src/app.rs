use crate::config::load_config;
use crate::input::InputHandler;
use crate::screen::Screen;
use crate::state::AppState;
use crate::extension_host::ExtensionHost;
use crate::spotlight::SpotlightModule;

use std::io::{stdout, Write};
use crossterm::terminal;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut state = AppState::new(config.clone());
    let spotlight = SpotlightModule::new(); // Assumes this exists
    let mut screen = Screen::new(config, spotlight);

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut ext_host = ExtensionHost::new();
    ext_host.load_all()?;

    let input = InputHandler;

    while state.is_running() {
        screen.draw(&mut screen_terminal_frame(), &mut state);
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

// Dummy placeholder for draw target
fn screen_terminal_frame<B: Backend>() -> Frame<B> {
    unimplemented!("This should return the real terminal frame for drawing.")
}
