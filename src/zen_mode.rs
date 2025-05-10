use ratatui::{Frame};
use ratatui::layout::Rect;
use crate::state::AppState;
use ratatui::widgets::{Block, Borders};

pub struct ZenModeState;

impl ZenModeState {
    pub fn new() -> Self {
        ZenModeState
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, _state: &AppState) {
        let block = Block::default().title("Zen Mode").borders(Borders::ALL);
        f.render_widget(block, area);
    }
}