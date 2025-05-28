## Code Changes

- Add `get_subtree_span(NodeID)` in layout.rs
- Modify `layout_recursive_safe()` to space siblings based on span width, not index
- Accumulate real width across children
- Respect `SIBLING_SPACING_X` as minimum gap between spans
- Center siblings around parent.x
- Add test: assert no horizontal overlap for nodes with long labels
