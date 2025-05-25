use ratatui::prelude::*;

pub trait Renderable {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect);
}
