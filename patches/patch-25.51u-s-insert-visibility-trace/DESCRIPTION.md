# Patch 25.51u-s – Insert Visibility Enforcement & Debug Trace

## Goals
- Ensure all inserted nodes are visually drawn or debug-logged
- Trace every insert’s parent, x/y, label, and role
- Add force-draw override for disconnected/fallback nodes
- Render ghost markers if node is unreachable but exists
