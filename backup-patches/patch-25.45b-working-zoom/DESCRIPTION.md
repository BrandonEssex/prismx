# Patch 25.45b – Working Zoom Scaling (Confirmed Inputs)

## Goals
- Fully implement visual zoom in/out using zoom_scale
- Keys: Ctrl+Alt+Z → Zoom In, Ctrl+Alt+X → Zoom Out, Ctrl+Alt+0 → Reset
- Ensure zoom_scale is applied to node position, links, and layout
- No visual side effects or overflow clipping

## Dependencies
- Requires CSI u mode enabled in iTerm2 for reliable input capture
- Depends on status_message for feedback and debugging

## Files
- `src/shortcuts.rs`: Add ZoomIn/Out/Reset mapped to Ctrl+Alt+Z/X/0
- `src/gemx/render.rs`: Apply zoom_scale to all node positions
- `src/state.rs`: Store and clamp zoom_scale
- Optional: Display zoom scale in status bar

