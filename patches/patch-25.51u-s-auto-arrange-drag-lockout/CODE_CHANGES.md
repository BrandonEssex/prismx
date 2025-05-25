## Code Changes

### 1. `src/gemx/interaction.rs`
- Import `std::time::Instant`
- In `start_drag()` return early when `auto_arrange` is `true`
  - Show status message "Drag disabled while auto-arrange is enabled"
  - Log a debug message when in debug mode
- In `drag_update()` skip all movement logic when `auto_arrange` is `true`
