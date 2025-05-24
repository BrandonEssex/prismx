## Code Changes

### 📦 .github/workflows/auto-merge-patch.yml

```yaml
name: Auto Merge Patch PRs

on:
  pull_request:
    types: [labeled, opened]

jobs:
  label-and-merge:
    if: startsWith(github.head_ref, 'patch-') && github.actor == 'BrandonEssex'
    runs-on: ubuntu-latest

    steps:
      - name: 🏷 Auto-label PRs from patch branches
        uses: actions-ecosystem/action-add-labels@v1
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          labels: auto-merge

      - name: 🤖 Enable auto-merge
        uses: peter-evans/enable-pull-request-automerge@v3
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          merge-method: squash
📦 .github/workflows/delete-merged-patches.yml
name: Delete Merged Patch Branches

on:
  pull_request:
    types: [closed]

jobs:
  cleanup:
    if: github.event.pull_request.merged == true && startsWith(github.head_ref, 'patch-')
    runs-on: ubuntu-latest
    steps:
      - name: 🔥 Delete Merged Patch Branch
        uses: dawidd6/action-delete-branch@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
