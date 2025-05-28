# Patch 25.49a – Auto-Arrange Fix + Spacing

## Goals
- Prevent nodes from stacking at root when auto-arrange is disabled
- Store and reuse last known manual positions
- Introduce grid-based fallback layout (like Tetris block layout)
- Add adjustable NODE_SPACING_X and NODE_SPACING_Y constants
- Prepare layout engine for future branch-color and separation logic

## Problem
Disabling auto-arrange with Ctrl+P currently causes all nodes to overlap at root.
This breaks visual structure and manual editing.

## Solution
- If a node was placed manually, retain its position when auto-arrange is off
- For newly added or root-free nodes, position them on a structured grid with default spacing
- Add constants or user-configurable values for X/Y spacing

