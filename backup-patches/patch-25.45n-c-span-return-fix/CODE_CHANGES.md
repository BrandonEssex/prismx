## Code Changes

- Modify layout_recursive_safe:
  - Track min_x and max_x as children are laid out
  - After all children are laid out, calculate node.x = (min_x + max_x) / 2
  - Return (min_x, max_x) upward for parent alignment
- Ensure no double assignment of node.x
- Do not modify layout of manual_coords nodes

## Tests
- Render unbalanced sibling sets
- Verify parent centers correctly
- Confirm no label overlaps or collapsed lines

