use ratatui::{backend::Backend, layout::Rect, Frame};

/// Render a small PrismX emblem in the top-right corner of the canvas.
/// Disabled to prevent stray emoji bleed on some terminals.
pub fn render_prism<B: Backend>(_: &mut Frame<B>, _: Rect) {}
