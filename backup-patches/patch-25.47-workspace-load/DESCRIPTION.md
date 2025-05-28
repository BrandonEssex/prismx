# Patch 25.47 – Workspace Switching + Save/Load

## Goals
- Add named workspaces to PrismX
- Allow switching between workspaces
- Add Save/Load from JSON file per workspace

## Hotkeys
- Ctrl+Tab: Switch to next workspace
- Ctrl+Shift+S: Save current workspace
- Ctrl+Shift+O: Load workspace from file

## Files
- `src/state/workspace.rs` (new): handles workspace sets
- `src/input/hotkeys.rs`
- `src/storage/json_io.rs` (new or updated)
