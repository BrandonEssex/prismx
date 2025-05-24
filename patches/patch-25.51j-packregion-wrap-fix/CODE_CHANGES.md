## Code Changes

- In `PackRegion::insert()`:
```rust
if self.x + w + margin > screen_width {
    self.x = 0;
    self.y += self.max_height + row_padding;
    self.max_height = 0;
}
In render_gemx() or layout_nodes():
Pass area.width to PackRegion::new(...)
Space out multiple root trees using horizontal layout
Fallback: if too large, wrap to next row
Added:
Tree cluster tiling logic
Free node row-fill packing
