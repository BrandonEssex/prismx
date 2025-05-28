## Code Changes

- In `layout_recursive_safe()`:
  - Track min_x and max_x during child layout pass
  - Compute parent_x = (min_x + max_x) / 2
  - Update node.x if auto_arrange is on
- Add fallback logic if node has no children (retain current x)

## Notes
- Do not update manual_coords if auto_arrange is off
- This preserves top-down balance as trees widen

