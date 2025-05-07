use crate::state::AppState;
use crate::screen::Screen;
use crate::input::Config;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

pub fn run_app() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let config = Config;
    let mut app_state = AppState::default();
    let mut screen = Screen::new();

    loop {
        terminal.draw(|f| screen.draw(f))?;

        if let Some(event) = config.poll_event()? {
            let continue_running =
                screen.handle_event(event.clone(), config.map(event.clone()), &mut app_state);
            if !continue_running {
                break;
            }
        }

        screen.update(&mut app_state);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}