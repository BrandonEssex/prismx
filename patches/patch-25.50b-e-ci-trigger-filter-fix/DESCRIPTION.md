# Patch 25.50b-e – Patch CI Trigger Filter Fix

## Goals
- Prevent GitHub patch-test CI from running for non-patch PRs
- Use `paths:` filter to limit triggering to patch folders only
- Add debug script to explain whether CI should run or not
