## Code Changes

- Add new crate `prismx_macros` for procedural macros
- Implement `#[derive(Widget)]` to auto-gen `impl Renderable for T`
- Add example usage for a dummy struct like `ZenPane`
- Derive macro generates a `fn draw(&self, frame: &mut Frame, area: Rect)` stub

Dependencies:
```toml
[dependencies]
syn = "2"
quote = "1"
proc-macro2 = "1"

[lib]
proc-macro = true
