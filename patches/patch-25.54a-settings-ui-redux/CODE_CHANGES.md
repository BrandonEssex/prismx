## Code Changes

### 1. Define Toggle Struct

```rust
struct SettingToggle {
    label: &'static str,
    is_enabled: fn(&AppState) -> bool,
    toggle: fn(&mut AppState),
}
2. Register Toggles
In AppState::new() or a settings_panel() fn:

[
    SettingToggle {
        label: "Auto-Arrange",
        is_enabled: |s| s.auto_arrange,
        toggle: |s| s.auto_arrange = !s.auto_arrange,
    },
    SettingToggle {
        label: "Debug Mode",
        is_enabled: |s| s.debug_input_mode,
        toggle: |s| s.debug_input_mode = !s.debug_input_mode,
    },
    SettingToggle {
        label: "Vertical Dock",
        is_enabled: |s| matches!(s.favorite_dock_layout, DockLayout::Vertical),
        toggle: |s| {
            s.favorite_dock_layout = match s.favorite_dock_layout {
                DockLayout::Vertical => DockLayout::Horizontal,
                _ => DockLayout::Vertical,
            };
        },
    },
]
3. Render Toggle UI
Highlight with arrow:

> [x] Auto-Arrange
  [ ] Debug Mode
Use border panel and cyan accent for highlight.

4. Key Handling
↑ and ↓ move selection
Space/Enter toggles current entry
5. Open with /settings or hotkey
