## Code Changes

- Add `src/logging.rs` with `log_debug!`, `log_warn!` macros
- Macros check `state.debug_input_mode`
- Used in:
  - spotlight command triggers
  - fallback root promotions
  - undo/redo stack transitions
