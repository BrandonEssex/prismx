## Code Changes

- Replace layout_recursive_safe with a true tidy layout:
  - First pass: measure span of all subtrees (label width + children)
  - Second pass: assign child Xs with fixed spacing, no depth inflation
- If parent has one child, align X without centering offset
- After layout, compute min_x and shift all nodes if negative
- Update tests to confirm zero-drift layout

## Key Constants
- SIBLING_SPACING_X = 3
- BASE_SPACING_X = 20
