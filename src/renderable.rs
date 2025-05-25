use ratatui::{Frame, layout::Rect, backend::Backend};

pub trait Renderable {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect);
}
