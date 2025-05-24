## Code Changes

- In layout_recursive_safe:
  - During child span calc, also calculate descendant depth
  - Multiply `depth_factor * MIN_NODE_GAP` and add to span
  - Clamp cursor_x increment to at least span + MIN_NODE_GAP
- Ensure spacing adapts to complex trees with deep children

## Notes
- Consider subtree depth when laying out
- Prevent spacing collapse when child span is 1 but subtree is tall

