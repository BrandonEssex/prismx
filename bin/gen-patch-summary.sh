#!/bin/bash
set -e

patch_dir="\$1"
[ -z "\$patch_dir" ] && echo "Usage: bin/gen-patch-summary.sh patch-folder-name" && exit 1

cd patches/\$patch_dir

{
  echo "# Patch Summary: \$patch_dir"
  echo
  echo "## Title"
  grep '^# Patch' DESCRIPTION.md || echo "(missing title)"
  echo
  echo "## Affected Modules"
  grep -i 'render_' CODE_CHANGES.md | cut -d: -f1 | sort -u || echo "(unspecified)"
  echo
  echo "## Commands Used"
  grep -oE '[a-z]+-[a-z]+' test_plan.sh || echo "(none found)"
  echo
  echo "## Notes"
  echo "_You can edit this file or provide it to Codex for summarization or history sync._"
} > PATCH_SUMMARY.md

echo "✅ PATCH_SUMMARY.md generated in patches/\$patch_dir"
