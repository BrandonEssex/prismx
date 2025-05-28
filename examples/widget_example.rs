use prismx_macros::Widget;
use prismx::render::traits::Renderable;

#[derive(Widget)]
struct MyWidget;

fn main() {
    let mut w = MyWidget;
    // Call method from Renderable to ensure impl exists
    w.tick();
}
