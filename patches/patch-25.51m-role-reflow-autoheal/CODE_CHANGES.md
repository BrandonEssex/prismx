## Code Changes

- New function: `recalculate_roles(&mut self)`
  - For each node:
    - If directly under a nearby node → assign `Child`
    - If horizontally aligned with nearby node → assign `Sibling`
    - Otherwise → `Free`

- In `render_gemx()`:
  - On `auto_arrange == true`, call:
    ```rust
    state.recalculate_roles();
    ```

- Updated drag logic:
  - On drop, trigger `recalculate_roles()` pass
  - Clamp to nearest node or free if no match

- Children now lock vertically below parent using:
  ```rust
  child.x = parent.x;
  child.y = parent.y + CHILD_SPACING_Y;
