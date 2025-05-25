## Code Changes

1. `layout/mod.rs`
   - `layout_nodes()` accepts `(term_width, term_height)` to define the layout area.
   - `layout_recursive_safe()` stacks child nodes vertically until the given
     height is exceeded, then shifts to the next column using
     `SIBLING_SPACING_X`.
   - Added `tracing::debug!` logs indicating when clusters wrap or overflow.

2. Call Sites
   - Updated `screen/gemx.rs`, `gemx/interaction.rs` and tests to pass the new
     height argument when invoking `layout_nodes()`.

