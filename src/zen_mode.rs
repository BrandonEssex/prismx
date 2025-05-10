use ratatui::{Frame};
use ratatui::layout::Rect;
use crate::state::AppState;

pub struct ZenModeState;

impl ZenModeState {
    pub fn new() -> Self {
        ZenModeState
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, _state: &AppState) {
        let block = ratatui::widgets::Block::default().title("Zen Mode").borders(ratatui::widgets::Borders::ALL);
        f.render_widget(block, area);
    }
}