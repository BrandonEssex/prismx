## Code Changes

- `tests/golden/render_gemx_snapshot.rs`
  - Renders a mock AppState using `render_gemx`
  - Compares against `tests/golden/gemx.snapshot`

- `bin/patch-risk-score.sh`
  - Flags any patch touching:
    - `beamx.rs`, `state.rs`, `app.rs`
  - Ensures test plan mentions:
    - “beam”, “triage”, “plugin”, or “render”

- `bin/gen-status-table.sh`
  - Reads `patches/*/PATCH_SUMMARY.md`
  - Generates a Markdown table of system health
  - Output: `STATUS_TABLE.md`

- `cargo test --test render_gemx_snapshot` is passing baseline
