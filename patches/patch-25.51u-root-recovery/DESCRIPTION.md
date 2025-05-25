# Patch 25.51u – Root Node Tracking + Tree Recovery Pass

## Goals
- Prevent layout_nodes() from failing due to missing roots
- Always ensure at least one node is in root_nodes
- Auto-promote unreachable nodes as fallback
- Visually mark unreachable nodes in dev mode
