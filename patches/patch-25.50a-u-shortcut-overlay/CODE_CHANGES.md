## Code Changes

- Add `src/ui/shortcuts.rs`:
```rust
pub const SHORTCUTS: &[(&str, &str)] = &[
    ("Ctrl+C", "Quit"),
    ("Ctrl+Z", "Undo"),
    ("Ctrl+Shift+Z", "Redo"),
    ("Ctrl+R", "Start/Finish Drag"),
    ("Ctrl+L", "Start/Finish Link"),
    ("Ctrl+N", "Free Node"),
    ("Tab", "Add Child"),
    ("Enter", "Add Sibling"),
    ("Ctrl+D", "Delete Node"),
    ("Ctrl+W", "Drill Down"),
    ("Ctrl+Q", "Pop Up"),
    ("Ctrl+T", "Toggle Collapse"),
    ("Ctrl+X", "Save Zen"),
    ("Ctrl+P", "Toggle Auto-Arrange"),
    ("Ctrl+G", "Snap Grid"),
    ("Alt+Left/Right", "Horizontal Scroll"),
    ("Ctrl+Space", "Module Switch"),
    ("Ctrl+.", "Settings"),
    ("Alt+Shift+S", "Spotlight"),
    ("Ctrl+Y", "Triage"),
    ("Ctrl+H", "Help / Show Overlay"),
    ("Esc", "Close Overlay")
];
```

- In `render_overlay.rs` or `render_help.rs`, use this list to generate `Paragraph<Lines>`.
- In `src/state.rs`, update `state.mode` mapping logic to:
```rust
impl From<&str> for BeamXMode {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zen" => BeamXMode::Zen,
            "triage" => BeamXMode::Triage,
            "spotlight" => BeamXMode::Spotlight,
            "settings" => BeamXMode::Settings,
            _ => BeamXMode::Default,
        }
    }
}
```

- In each render:
```rust
let beamx = BeamX {
    tick,
    enabled: true,
    style: BeamXStyle::from(BeamXMode::from(&state.mode)),
};
```

- Add dev script `bin/gen-cheatsheet-from-overlay.sh`
