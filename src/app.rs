use crate::config::load_config;
use crate::extension_host::ExtensionHost;
use crate::input::handle_input;
use crate::screen::Screen;
use crate::state::AppState;
use crossterm::terminal;
use std::io::{self, stdout};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut state = AppState::new(config);
    let mut screen = Screen::new()?;

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut extension_host = ExtensionHost::new();
    extension_host.load_all()?;

    while state.is_running() {
        screen.draw(&mut stdout, &mut state)?;
        handle_input(&mut state)?;
    }

    crossterm::execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
