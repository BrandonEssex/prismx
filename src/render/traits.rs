use ratatui::{prelude::*, Frame};

/// Trait for types that can render a UI frame and progress animations.
pub trait Renderable {
    /// Render the component to the given area.
    fn render_frame<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);

    /// Advance internal state for the next frame.
    fn tick(&mut self);
}

/// Marker trait for UI widgets.
pub trait Widget: Renderable {}

