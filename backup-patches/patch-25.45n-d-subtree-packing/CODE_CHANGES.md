## Code Changes

- In layout_recursive_safe:
  - First pass: for each child, call layout_recursive_safe to get its width/span
  - Accumulate width + SIBLING_SPACING_X into offset
  - Second pass: assign child_x = parent_x - (total_width / 2) + cumulative_offset
- Remove fixed index-based offset math
- Add test case: wide-left subtree, narrow-right sibling — spacing should remain correct

## Notes
- This mirrors void-rs Pack logic

