# Patch 25.51u-c – Final Root Promotion Dedup & Log Suppression

## Goals
- Prevent `ensure_valid_roots()` from logging or pushing duplicates
- Ensure `render_gemx()` only promotes unreachable nodes once
- Suppress fallback spam under normal operation
- Only log unreachable recovery during debug_input_mode
