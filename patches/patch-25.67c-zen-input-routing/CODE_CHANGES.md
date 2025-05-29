## Code Changes

- Add global match: if `state.mode == Mode::Zen`, route key to `zen::editor::handle_key`
- Add trace log in `editor.rs` to confirm Char/Enter are received
- Ensure typing inserts a `ZenJournalEntry`
