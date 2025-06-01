## Code Changes

- Add `bin/verify-settings.sh` to:
  - Parse toggles from `toggles.rs`
  - Search for `settings.toggle_name` in `/src/`
  - Report unused toggles
- Fail CI if a toggle is defined but not used
