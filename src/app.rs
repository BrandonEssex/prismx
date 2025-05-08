use std::io::{stdout};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::input::Config;
use crate::logger::init_logging;
use crate::screen::Screen;
use crate::state::AppState;

pub fn run_app() -> anyhow::Result<()> {
    init_logging();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut screen = Screen::new();
    let config = Config;
    let mut app_state = AppState::new("data".into());

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
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}