## Code Changes

- Added `node_max_width` and `NODE_WRAP_LABELS` to `src/theme/layout.rs`.
- Updated GemX renderer to clamp node labels using new width limit.
- Optional soft wrap when `NODE_WRAP_LABELS` is enabled.
- Connectors now anchor using clamped widths to avoid jitter.
