// Author: Brandon Essex
// Entry point for PrismX / GemX

mod action;
mod input;
mod node_tree;
mod state;
mod ui;
mod config;
mod plugin;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::state::AppState;
use crate::ui::draw::draw;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::default();

    let res = run_app(&mut terminal, &mut app_state);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app_state: &mut AppState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, app_state))?;
        if app_state.should_quit {
            break;
        }
        app_state.handle_input_event()?;
    }
    Ok(())
}
