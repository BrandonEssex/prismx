use ratatui::{prelude::*, Frame};

use crate::state::AppState;
use crate::render::traits::Renderable;
use crate::zen::view::render_zen;

/// Wrapper struct to render the Zen editor view.
pub struct ZenView<'a> {
    pub state: &'a AppState,
}

impl<'a> ZenView<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl<'a> Renderable for ZenView<'a> {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        render_zen(f, area, self.state);
    }
}
