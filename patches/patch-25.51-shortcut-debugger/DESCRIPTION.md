# Patch 25.51 – Shortcut Debugger + Manager

## Goals
- Print key events with modifier info to a status bar (not stdout)
- Enable/disable debug input logging with Ctrl+Shift+D
- Centralize shortcut mapping in `shortcuts.rs` using `match_shortcut()`

## Modules
- `src/shortcuts.rs`: defines all hotkeys in one place
- `src/state.rs`: adds `debug_input_mode` and `status_message`
- `src/input/hotkeys.rs`: logs keypress into `status_message`
- `render.rs`: show `status_message` at bottom of screen

