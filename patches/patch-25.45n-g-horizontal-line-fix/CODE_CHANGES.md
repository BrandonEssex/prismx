## Code Changes

- In layout_recursive_safe (layout.rs):
  - Remove per-child assignment of `child_y`
  - Move `let child_y = y + 1` before the loop
  - Apply same Y to all siblings
- Confirm visual flattening of children in TUI

## Test
- Add many siblings to one node
- Expect them to fan out left/right, not down


