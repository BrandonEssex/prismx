# Patch 25.51u-z – Recursion Proof via Visited Trace & Depth Guard

## Goals
- Prevent infinite recursion in auto-arrange mode
- Use visited set to detect cycles during layout
- Clamp max layout depth
- Avoid stack overflow on child loops or corrupted graphs
