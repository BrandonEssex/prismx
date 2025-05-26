## Code Changes

- Fixed `add_child_node()` to ensure valid parent linkage
- Added recursion clamp to prevent runaway auto-layout
- Improved spacing logic for parent-child hierarchy
- If no valid root node: fallback gracefully to topmost node
- Added debug log tracepoints when layout fails
