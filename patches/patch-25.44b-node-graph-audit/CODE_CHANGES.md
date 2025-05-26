## Code Changes

- Adds `state.audit_node_graph()` utility
- Called on startup and after node creation (Enter, Tab, Ctrl+N/B)
- Detects and logs:
  - Cycles
  - Missing parents
  - Orphans
  - Duplicate children
- Optional: auto-heals fixable structure issues
