## Code Changes

- In `.github/workflows/auto-merge-patch.yml`, replace:

```yaml
on:
  pull_request:
    types: [labeled, opened]
With:

on:
  pull_request:
    types: [labeled, opened, synchronize]
This ensures:
PRs trigger merge when opened
PRs trigger merge when updated
PRs re-checked when new commits are pushed
