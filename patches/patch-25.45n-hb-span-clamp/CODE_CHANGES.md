## Code Changes

- In `layout_recursive_safe()`:
  - Before pushing `(child_id, span)`, compute:
    ```rust
    let label_w = nodes[&child_id].label.len() as i16 + 2;
    let span = subtree_span.max(label_w + MIN_NODE_GAP);
    ```

- This ensures each child gets enough visual space
- Update sibling cursor math to space by clamped span
