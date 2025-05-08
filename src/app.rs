use std::path::PathBuf;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    input::Config,
    logger::init_logging,
    screen::Screen,
    state::AppState,
};

pub fn run_app() -> anyhow::Result<()> {
    init_logging();

    let config = Config;
    let data_dir = PathBuf::from("data");
    let mut app_state = AppState::new(data_dir);
    let mut screen = Screen::new();

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let running = loop {
        terminal.draw(|f| screen.draw(f))?;

        if let Some(event) = config.poll_event()? {
            let action = config.map(&event);
            let continue_running = screen.handle_event(&event, action, &mut app_state);
            if !continue_running {
                break;
            }
        }
    };

    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}