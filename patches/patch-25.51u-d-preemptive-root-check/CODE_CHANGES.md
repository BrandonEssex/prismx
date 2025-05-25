## Code Changes

### 1. In `handle_tab()` or child insertion function:
```rust
state.ensure_valid_roots(); // Run BEFORE any layout or insert logic
This ensures root_nodes has at least one valid entry before layout, role calc, or drawing.
2. Optional: Assert root_nodes after fallback:
debug_assert!(!state.root_nodes.is_empty());
3. Ensure no duplicate fallback logs by keeping logic from 25.51u-c
