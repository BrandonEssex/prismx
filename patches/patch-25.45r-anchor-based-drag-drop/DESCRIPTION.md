# Patch 25.45r – Anchor-Based Dragging & Drop-to-Canvas

## Goals
- Detect and prevent invalid reparenting cycles
- Allow dropping a node to canvas to create a new free root
- Fallback to assigning manual_coords if no valid target
- Support drag-and-drop interaction without graph corruption
