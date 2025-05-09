use crate::config::load_config;
use crate::screen::Screen;
use crate::state::AppState;
use crate::input::{InputHandler};
use crate::actions::Action;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut state = AppState::new(); // fixed: removed config.clone()

    enable_raw_mode()?;
    let mut screen = Screen::new(config);
    let input = InputHandler;

    loop {
        screen.draw(&mut state);

        if let Some(event) = input.poll_event()? {
            if let Some(action) = input.handle_event(event) {
                match action {
                    Action::Quit => state.quit(),
                    _ => screen.handle_action(action),
                }
            }
        }

        if !state.is_running() {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}