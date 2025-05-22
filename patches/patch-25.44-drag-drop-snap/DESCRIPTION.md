# Patch 25.44 – Drag-and-Drop + Snap-to-Grid

## Goals
- Enable drag-and-drop repositioning of mindmap nodes (GemX module)
- Implement optional "Snap-to-Grid" behavior for free nodes
- Retain position persistence during layout and Drill/Pop cycles

## Hotkeys / Controls
- Click + drag node with mouse or keyboard nav (if TUI-compatible)
- Snap-to-grid toggled with Ctrl+G (new hotkey)

## Modules
- `src/gemx/interaction.rs` (new or existing input handler)
- `src/gemx/render.rs` (reposition & grid logic)
- `src/input/hotkeys.rs` (add Ctrl+G toggle)

## Visual Behavior
- Drag mode moves node and children together
- Grid snapping aligns X/Y to nearest 20px spacing

## Related
- Builds on horizontal layout and auto-arrange (Patch 25.42)
- Should not break Ctrl+N (new node) or Ctrl+B (add branch)
