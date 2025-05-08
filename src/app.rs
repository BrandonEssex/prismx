use crate::input::{Action, Config};
use crate::logger::init_logging;
use crate::screen::Screen;
use crate::state::AppState;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;
use std::path::PathBuf;

pub fn run_app() -> anyhow::Result<()> {
    init_logging()?;

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut screen = Screen::new();
    let mut app_state = AppState::new(PathBuf::from("data/"));
    let config = Config;

    loop {
        terminal.draw(|f| screen.draw(f))?;

        if let Some(event) = config.poll_event()? {
            let action = config.map(event);
            let continue_running = screen.handle_event(event, action, &mut app_state);
            if !continue_running {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}