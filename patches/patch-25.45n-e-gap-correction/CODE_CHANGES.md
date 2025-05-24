## Code Changes

- In layout_recursive_safe:
  - For each child, measure label width (e.g. `child.label.len()`)
  - During span accumulation, compute:
    `child_span = max(subtree_width, label_width) + MIN_NODE_GAP`
  - Add child_span to offset
  - Layout all children with proper spacing before centering block

## Notes
- Prevents label collisions and visual collapse
- All children should appear distinct even if their trees are shallow


