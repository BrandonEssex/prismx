use ratatui::{Frame, Terminal};
use crate::state::AppState;

pub struct Screen;

impl Screen {
    pub fn new() -> Self {
        Screen
    }

    pub fn run<B>(&mut self, _terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
    where
        B: ratatui::backend::Backend,
    {
        Ok(())
    }

    pub fn draw<'a>(&mut self, f: &mut Frame<'a>, _state: &mut AppState) {
        // Rendering logic goes here
    }
}