// FINAL FULL FILE DELIVERY
// Filename: /src/app.rs
// File Delivery Progress: 2/∞ FINAL FILES delivered

use std::io::{stdout, Write};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::config::load_config;
use crate::logger;
use crate::screen::Screen;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    logger::init_logger(&config)?;

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