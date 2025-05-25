## Code Changes

### 1. Fix Icon Mapping

Update `module_icon(mode)` to:

```rust
fn module_icon(mode: Mode) -> &'static str {
    match mode {
        Mode::GemX => "💭",      // Mindmap
        Mode::Zen => "🧘",       // Zen Mode
        Mode::Triage => "🧭",    // Triage
        Mode::Spotlight => "🔍", // Spotlight
        Mode::Settings => "⚙️",  // Settings
        _ => "❓",
    }
}
2. Update Module Switcher Item List
Replace hardcoded or mismatched labels with:

[
    ("💭", "Mindmap", Mode::GemX),
    ("🧘", "Zen Mode", Mode::Zen),
    ("🧭", "Triage", Mode::Triage),
    ("🔍", "Spotlight", Mode::Spotlight),
    ("⚙️", "Settings", Mode::Settings),
]
Render with active highlight, padding, and clean alignment.

3. Update Status Bar and Mode Panel
Ensure Mode::Settings is labeled correctly:

match state.mode {
    Mode::Settings => "settings",
    Mode::Spotlight => "spotlight",
    ...
}
Prevent fallback to incorrect label like "spotlight" when in Settings view.

