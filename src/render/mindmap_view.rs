use ratatui::{prelude::*, Frame};

use crate::render::traits::Renderable;

/// Placeholder view for future mindmap rendering support.
pub struct MindmapView;

impl Renderable for MindmapView {
    fn render<B: Backend>(&self, _f: &mut Frame<B>, _area: Rect) {
        // Rendering not yet implemented
    }
}
