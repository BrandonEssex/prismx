use ratatui::Terminal;
use ratatui::Frame;
use ratatui::backend::Backend;

use crate::state::AppState;

pub struct Screen;

impl Screen {
    pub fn new() -> Self {
        Screen
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|f| self.draw(f, &mut AppState::default()))?;
        Ok(())
    }

    pub fn draw<'a>(&mut self, f: &mut Frame<'a>, _state: &mut AppState) {
        // TODO: Actual drawing logic will go here
    }
}