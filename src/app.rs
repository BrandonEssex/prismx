use crate::input::Config;
use crate::logger::init_logging;
use crate::screen::Screen;
use crate::state::AppState;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

pub fn run_app() -> anyhow::Result<()> {
    init_logging()?;
    let config = Config;
    let mut screen = Screen::new();
    let mut app_state = AppState::default();

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|f| screen.draw(f))?;

        if let Some(event) = config.poll_event()? {
            let continue_running =
                screen.handle_event(event.clone(), config.map(event), &mut app_state);
            if !continue_running {
                break;
            }
        }
    }

    Ok(())
}