# Patch 25.45a – Zoom Repair + Boundary Clamp

## Goals
- Fix zoom in/out/reset not responding to modifier hotkeys
- Implement zoom-to-cursor (center on selected node)
- Ensure zoom scale is applied visually in render
- Prevent nodes from flashing or overlapping canvas edges

## Hotkeys (must use modifiers)
- Alt+= : Zoom In
- Alt+- : Zoom Out
- Alt+0 : Reset Zoom / Zoom-to-Cursor

## Affected Modules
- `src/input/hotkeys.rs` → Hotkey registration
- `src/gemx/render.rs` → Apply zoom_scale to all layout
- `src/gemx/view.rs` or `state.rs` → Centering logic and zoom state

