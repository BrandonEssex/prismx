use crate::config::load_config;
use crate::screen::Screen;
use crate::input::InputHandler;
use crate::state::AppState;
use crate::spotlight::SpotlightModule;
use crate::actions::Action;
use crate::logger;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    logger::init_logger(&config);

    let mut state = AppState::new();
    let spotlight = SpotlightModule::new();
    let mut screen = Screen::new(config.clone(), spotlight);

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let backend = ratatui::backend::CrosstermBackend::new(&mut stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let input = InputHandler;

    loop {
        terminal.draw(|f| {
            screen.draw(f, &mut state);
        })?;

        if let Some(event) = input.poll_event()? {
            if let Some(action) = input.handle_event(event) {
                match action {
                    Action::Quit => {
                        state.quit();
                        break;
                    }
                    _ => screen.handle_action(action, &mut state),
                }
            }
        }

        if !state.is_running() {
            break;
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}