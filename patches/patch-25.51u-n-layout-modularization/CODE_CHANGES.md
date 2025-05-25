## Code Changes

- Create `layout/roles.rs`:
  - Contains `recalculate_roles()`
- Create `layout/fallback.rs`:
  - Handles fallback root promotion and layout recovery
- Add `mod.rs` in layout to re-export functions
- Remove role logic from `mod.rs` and `state/mod.rs`
