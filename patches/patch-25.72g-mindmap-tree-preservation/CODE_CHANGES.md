## Code Changes

- Identify “active root” (current focus or selected node's parent)
- Lock that subtree’s layout in-place
- Push other subtrees away to avoid collision, never shift the active branch
- Animate or smoothly reposition sibling groups if needed
