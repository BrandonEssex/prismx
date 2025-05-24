# Patch 25.45p – Focus Stack + Zoom Context Restoration

## Goals
- Implement focus stack for zoom/drilldown view memory
- Preserve scroll_x, scroll_y and selected node when zooming in
- Restore previous context on pop-up (Ctrl+Q)
- Prevent viewport jumps and improve visual stability
