## Code Changes

- Add guard in `layout_nodes()` to skip layout pass if:
  - Parent node is undefined
  - Newly added node hasn’t been fully linked
- Prevent panic or overwrite of root state
- Log fallback triggers with context (e.g. “insert off-screen – skipping layout”)
- Ensure that auto-arrange disables itself if layout fails consecutively
