use ratatui::{prelude::*, Frame};

/// Trait for things that can be drawn to the terminal.
pub trait Renderable {
    /// Render the component to the given area.
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect);
}

