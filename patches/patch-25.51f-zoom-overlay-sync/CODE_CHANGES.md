## Code Changes

- `src/ui/shortcuts.rs`:
```rust
("Alt+=", "Zoom In"),
("Alt+-", "Zoom Out"),
("/zoom in", "Zoom In via Spotlight"),
("/zoom out", "Zoom Out via Spotlight"),
render_shortcuts_overlay.rs:
Re-renders shortcut list dynamically
Re-run:
bin/gen-cheatsheet-from-overlay.sh
