use ratatui::{backend::CrosstermBackend, layout::Rect, Frame};
use std::io::Stdout;

pub type PluginFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait PluginRender {
    fn render(&mut self, f: &mut PluginFrame<'_>, area: Rect);
}
