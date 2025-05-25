# Patch 25.51u-y – Insert-Time Link Check + Recursion Clamp

## Goals
- Prevent circular insertions (self-parent or self-child)
- Apply immediate spacing to new children when auto_arrange is off
- Clamp layout recursion to avoid stack overflows
- Log recursion failures in debug mode
