# Patch 25.45n-f – Span Depth Buffer + Final Sibling Jitter Fix

## Goals
- Ensure spacing between siblings accounts for label width + descendant depth
- Prevent horizontal jitter or overlap in wide multi-level branches
- Clamp all child spans with buffer based on depth

## Dependencies
- Final fix after 25.45n-e: apply at layout pass not just during draw

