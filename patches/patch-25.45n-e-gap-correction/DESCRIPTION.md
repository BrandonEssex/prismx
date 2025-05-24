# Patch 25.45n-e – Enforce Label Width + Minimum Sibling Gap

## Goals
- Prevent siblings from overlapping horizontally
- Consider each node’s label width when spacing siblings
- Enforce `MIN_NODE_GAP` (default: 3 characters) between siblings
- Align children under parent only after spacing is resolved

## Dependencies
- Follows 25.45n-d: span-aware layout
- Corrects accumulated offset spacing

## Constants
- BASE_LABEL_WIDTH estimate or computed from label
- MIN_NODE_GAP = 3

