## Code Changes

- In `.github/workflows/prismx-patch-tests.yml`:

1. Add this at the top:
```yaml
concurrency:
  group: patch-tests-${{ github.ref }}
  cancel-in-progress: true
Replace the trigger block with:
on:
  pull_request:
    branches:
      - main
    paths:
      - 'patches/patch-*/*'
Add fallback step to patch-test job:
- name: 🔁 Skip if no patches found
  if: steps.set-matrix.outputs.matrix == '[]'
  run: echo "🟡 No valid patches. Matrix skipped."
