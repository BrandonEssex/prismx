// src/app.rs (clean launcher wrapper)
use crate::screen::Screen;
use crate::state::AppState;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;

pub fn launch() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let app_state = AppState::default();

    let mut screen = Screen::new(terminal, app_state);
    let result = screen.run();

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    result
}