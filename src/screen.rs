use ratatui::{Terminal, Frame};
use ratatui::backend::Backend;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

use crate::state::AppState;

pub struct Screen;

impl Screen {
    pub fn new() -> Self {
        Screen
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = AppState::default();

        loop {
            terminal.draw(|f| self.draw(f, &mut state))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    // Future: Add routing to handle key events here
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, _f: &mut Frame, _state: &mut AppState) {
        // TODO: render actual components
    }
}