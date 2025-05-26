use ratatui::{backend::Backend, layout::Rect, Frame};

/// Render a small PrismX emblem in the top-right corner of the canvas.
pub fn render_prism<B: Backend>(_: &mut Frame<B>, _: Rect) {
    // Icon rendering removed – previously drew a debug prism glyph
}
