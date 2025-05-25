# Patch 25.51u-g – Final Fallback + Node Insertion Stabilization

## Goals
- Fix sibling insertion causing unreachable or orphaned nodes
- Prevent fallback from re-promoting already-rooted or invalid nodes
- Auto-position free nodes when auto-arrange is disabled
- Ensure all fallback logic respects deduplication and visual intent
