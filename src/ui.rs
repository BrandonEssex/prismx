use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use crate::dashboard::render_dashboard;
use crate::keymap::{get_command, Command};
use crate::dashboard_toggle::should_render_dashboard;

pub fn run_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut show_dashboard = true;

    loop {
        terminal.draw(|f| {
            if should_render_dashboard(show_dashboard) {
                render_dashboard(f);
            }
            // future: render mindmap, zen, plugin slots here
        })?;

        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match get_command(key_event) {
                    Some(Command::Quit) => break,
                    Some(Command::NewNode) => {
                        println!("(todo) Create new node");
                    }
                    Some(Command::CutNode) => {
                        println!("(todo) Cut node");
                    }
                    Some(Command::OpenSettings) => {
                        println!("(todo) Open settings UI");
                    }
                    Some(Command::OpenSpotlight) => {
                        println!("(todo) Activate spotlight search");
                    }
                    Some(Command::OpenDashboard) => {
                        show_dashboard = !show_dashboard;
                    }
                    None => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
