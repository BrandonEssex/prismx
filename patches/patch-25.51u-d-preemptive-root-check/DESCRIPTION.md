# Patch 25.51u-d – Preemptive Root Check on Tab Insert

## Goals
- Prevent crash on Tab insert by validating roots **before** layout
- Ensure fallback promotion logic runs before role recalculation
- Lock in layout stability across all input handlers
