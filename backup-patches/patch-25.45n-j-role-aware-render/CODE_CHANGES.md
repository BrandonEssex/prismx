## Code Changes

- In `render_gemx()`:
  - Unpack `let (layout, roles) = layout_nodes(...)`
  - Match each node’s `LayoutRole` to apply:
    - `Ghost` → skip drawing
    - `Free` → standard style
    - `Root` → bold or colored
    - `Orphan` → warning underline (optional)

- Update any call site still using just `layout = layout_nodes(...)` without unpacking

## Optional:
- Draw symbols or badges next to roles: 🧩 Free, 🧭 Root, ⚠️ Orphan
