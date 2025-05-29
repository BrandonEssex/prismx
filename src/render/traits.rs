use ratatui::{prelude::*, Frame};
use ratatui::backend::CrosstermBackend;
use std::io::Stdout;

/// Concrete frame type used throughout the UI.
pub type RenderFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Trait for lightweight view objects that know how to draw themselves.
pub trait Renderable {
    /// Render the component to the given area.
    fn render(&mut self, f: &mut RenderFrame<'_>, area: Rect);
}

/// Marker trait for UI widgets.
pub trait Widget: Renderable {}

