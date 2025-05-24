## Code Changes

- Add `PackRegion` utility:
  ```rust
  struct PackRegion {
      cursor_x: i16,
      cursor_y: i16,
      row_height: i16,
      term_width: i16,
  }
In render_gemx() or wherever layout_nodes() is called:
Measure (width, height) of each root via subtree_span
Assign (x, y) via PackRegion.insert((w, h))
Offset each subtree layout by that anchor
Update draw logic to reflect shifted coordinates
Result:

Layout fills screen horizontally (and wraps downward)
Roots never overlap again
PrismX can handle large, multi-cluster documents
