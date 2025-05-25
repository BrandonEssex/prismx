# Patch 25.51u-f – Fallback Role Debounce + Visibility Check

## Goals
- Stop infinite fallback loops during unreachable node promotion
- Only promote **one** unreachable node per render frame
- Avoid promoting nodes that cannot lead to visual layout (no children)
- Debounce fallback logic so it doesn't re-fire unless root state changes
