# Patch 25.51u-e – Root Promotion Lock & Fallback Guard

## Goals
- Prevent repeated fallback promotion of the same node
- Track last-promoted root to suppress log spam
- Ensure fallback does not trigger every render cycle
