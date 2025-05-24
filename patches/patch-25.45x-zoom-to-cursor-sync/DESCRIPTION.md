# Patch 25.45x – Zoom-to-Cursor Sync Engine

## Goals
- Center view on selected node during zoom (zoom-to-cursor)
- Maintain consistent layout using zoom_scale transform
- Prevent clipping, overflow, and drift during zoom transitions
- Update drag and click logic to compensate for zoom
- Clamp scroll_x/scroll_y to visible content bounds

## Constants
- zoom_scale: f32
- BASE_SPACING_X / Y
- MIN_NODE_GAP

