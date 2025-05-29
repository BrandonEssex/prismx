use prismx::render::traits::{Renderable, RenderFrame};
use ratatui::prelude::*;

struct MyWidget;

impl Renderable for MyWidget {
    fn render(&mut self, _f: &mut RenderFrame<'_>, _area: Rect) {}
}

fn main() {
    // Example is compile-time only
}
