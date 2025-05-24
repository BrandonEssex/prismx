# Patch 25.50b-d – Auto-Merge Trigger Fix

## Goals
- Fix skipped auto-merge job when patch PRs are updated
- Add `synchronize` event to PR workflow trigger
- Ensure PRs from `patch-*` branches auto-merge when all checks pass
