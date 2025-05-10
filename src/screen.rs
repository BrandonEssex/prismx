use ratatui::{Frame, Terminal};
use ratatui::backend::Backend;
use crate::state::AppState;

pub struct Screen;

impl Screen {
    pub fn new() -> Self {
        Screen
    }

    pub fn run<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
    where
        B: Backend,
    {
        // Replace with your TUI main loop
        terminal.draw(|f| self.draw(f, &mut AppState::default()))?;
        Ok(())
    }

    pub fn draw<'a>(&mut self, f: &mut Frame<'a>, _state: &mut AppState) {
        // Example placeholder: replace with real draw logic
    }
}