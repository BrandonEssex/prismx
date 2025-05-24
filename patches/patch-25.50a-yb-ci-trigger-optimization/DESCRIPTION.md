# Patch 25.50a-yb – Patch Test CI Trigger Optimization

## Goals
- Prevent patch-test CI from running on all patches on every push
- Limit patch test runs to PRs that modify patch folders
- Add fallback for empty matrix to prevent false failures
- Enable concurrency auto-cancel on redundant patch CI runs
