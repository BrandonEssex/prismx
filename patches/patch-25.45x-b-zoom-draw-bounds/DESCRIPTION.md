# Patch 25.45x-b – Zoom Draw Clamp + Bounds Fix

## Goals
- Prevent zoomed nodes from crashing ratatui buffer rendering
- Clamp draw_x / draw_y within screen dimensions
- Cap zoom factor to avoid runaway scaling
- Ensure arrow rendering respects screen bounds

