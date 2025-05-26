## Code Changes

- Move Zen-related functions to `state/zen.rs`
- Move `undo()`, `redo()`, `push_undo()` to `state/undo.rs`
- Move navigation and focus ops to `state/focus.rs`
- Keep `AppState` struct and `impl Default` in `mod.rs`
- Use `pub mod zen;`, `mod focus;`, etc. from `mod.rs`
