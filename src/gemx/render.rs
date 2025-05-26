use ratatui::{prelude::*, Frame};

use crate::state::AppState;
use crate::render::traits::Renderable;
use crate::screen::gemx::render_gemx;

/// Wrapper implementing [`Renderable`] for the GemX screen.
pub struct GemxRenderer<'a> {
    pub state: &'a mut AppState,
}

impl<'a> GemxRenderer<'a> {
    pub fn new(state: &'a mut AppState) -> Self {
        Self { state }
    }
}

impl<'a> Renderable for GemxRenderer<'a> {
    fn render_frame<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        // Render main GemX view
        render_gemx(f, area, self.state);
    }

    fn tick(&mut self) {}
}

