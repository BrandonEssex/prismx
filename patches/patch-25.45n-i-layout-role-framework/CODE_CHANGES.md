## Code Changes

- Add enum `LayoutRole` in layout.rs or node.rs:
  ```rust
  enum LayoutRole {
      Root,
      Child,
      Free,
      Orphan,
      Anchor,
      Ghost,
      Portal,
  }
Option 1 (recommended): store LayoutRole in a new map during layout
Option 2: add role: LayoutRole to Node struct (if roles will persist)
In layout_nodes() or layout_recursive_safe():
Assign role per node based on:
Parent presence
Auto-arrange flag
Manual override
Drill/Focus state
