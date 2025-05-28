# Patch 25.45 – Zoom Scaling + Zoom-to-Cursor

## Goals
- Implement zoom in/out functionality in the GemX canvas
- Allow resetting zoom level to 100% (default scale)
- Add Zoom-to-Cursor support: canvas centers on selected node while zooming

## Hotkeys
- Ctrl+= → Zoom In
- Ctrl+- → Zoom Out
- Ctrl+0 → Reset Zoom

## Affected Files
- `src/gemx/render.rs`: update draw scale logic
- `src/gemx/view.rs`: track zoom level and canvas offset
- `src/input/hotkeys.rs`: register zoom commands

## Constraints
- Avoid breaking horizontal scroll or auto-arrange logic
- Must maintain node visibility during zoom

## Builds On
- Patch 25.42 (horizontal layout)
- Patch 25.44 (drag/drop positioning)
