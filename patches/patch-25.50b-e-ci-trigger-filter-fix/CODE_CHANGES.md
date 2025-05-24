## Code Changes

- In `.github/workflows/prismx-patch-tests.yml`:

Change trigger block:
```yaml
on:
  pull_request:
    branches:
      - main
To:

on:
  pull_request:
    branches:
      - main
    paths:
      - 'patches/patch-*/*'
Add bin/ci-verify-scope.sh:
#!/bin/bash
echo "🧪 Checking if this PR touches a patch folder:"
git diff --name-only origin/main...HEAD | grep '^patches/patch-' && echo "✅ Patch change detected" && exit 0
echo "🛑 No patch folder changes found. Patch-test CI not required."
exit 1
