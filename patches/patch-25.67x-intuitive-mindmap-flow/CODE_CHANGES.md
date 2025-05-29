## Code Changes

- Place children vertically beneath parents with slight horizontal offset
- Align siblings horizontally with spacing tolerance
- Draw connector lines between parent/child and sibling groups
- Add drag-to-reparent logic:
  - Drop on node → becomes child
  - Drag out → becomes sibling or free
  - Orphan adoption by nearest ancestor or root rebalance
- Update layout cache to reflect tree structure immediately
