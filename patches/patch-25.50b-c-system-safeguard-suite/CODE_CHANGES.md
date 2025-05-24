## Code Changes

- Added `tests/golden/render_gemx_snapshot.rs`:
  - Renders a mock `AppState` to `ratatui::Buffer`
  - Compares it to `tests/golden/gemx.snapshot`
  - Fails test if it doesn't match

- Added `bin/patch-risk-score.sh`:
  - Flags patches touching high-risk files like `beamx.rs`, `state.rs`, `app.rs`
  - Fails if no matching test plan coverage keywords are found

- Added `bin/gen-status-table.sh`:
  - Aggregates all `PATCH_SUMMARY.md` into a Markdown table
  - Outputs to `STATUS_TABLE.md` in root

- Golden test can be run via `cargo test --test render_gemx_snapshot`
