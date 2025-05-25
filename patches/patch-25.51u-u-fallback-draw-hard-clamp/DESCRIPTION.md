# Patch 25.51u-u – Fallback Root Draw Hard Clamp

## Goals
- Assign guaranteed coordinates to any promoted root node
- Force render injection with safe bounds
- Prevent infinite fallback retry or draw spam
- Stop layout from writing out-of-bounds Rects
