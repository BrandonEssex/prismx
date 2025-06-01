## Code Changes

- Reserve bottom 2 rows for input block
- On entry submit:
  - Set `scroll_offset = entries.len().saturating_sub(visible)`
- Render input bar as static footer (always in frame)
