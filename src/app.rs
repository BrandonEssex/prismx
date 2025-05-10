// FINAL FULL FILE DELIVERY
// Filename: /src/app.rs
// File Delivery Progress: FINAL BUILD PATCH

use std::io::stdout;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::config::load_config;
use crate::logger::init_logger;
use crate::screen::Screen;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    init_logger(&config)?;

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut screen = Screen::new();
    screen.run(&mut terminal)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}