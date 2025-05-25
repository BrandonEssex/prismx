# Patch 25.51u-p – Fallback Label Clamp + Render Safety

## Goals
- Prevent infinite `[F]` prefix spam on fallback-promoted nodes
- Clamp fallback label to avoid memory or draw overflows
- Force fallback-promoted nodes into the layout
- Stop fallback from retrying on already handled nodes
