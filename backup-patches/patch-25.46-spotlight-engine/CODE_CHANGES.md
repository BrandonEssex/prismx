## Code Changes

- Create `src/spotlight.rs`
  - Spotlight struct with input, mode, results, and selected index
  - SpotlightMode enum: Command, Node, Tag, Jump
  - SpotlightResult enum: Command(name), Node(id), Tag(name), Jump(id)
- In tui/mod.rs:
  - Add input capture when spotlight is open
  - Render result list, highlight selected
- In app state:
  - Add `show_spotlight: bool`, `spotlight: Spotlight`
- Add registry of actions (struct SpotlightAction with name, type, handler fn)
- Fuzzy matching + result sort (placeholder with `strsim` or manual scoring)
