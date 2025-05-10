use std::io::{stdout};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::screen::Screen;

pub fn launch() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut screen = Screen::new();
    screen.run(&mut terminal)?;

    disable_raw_mode()?;
    Ok(())
}