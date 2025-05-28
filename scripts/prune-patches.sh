#!/bin/bash
set -euo pipefail

echo "🔍 Cleaning unused or duplicate patch folders..."

# 1. Prune empty dirs
find patches -type d -name 'patch-*' -empty -print -exec rmdir {} \;

# 2. Archive known junk patches (manually curated)
ARCHIVE=patches/_archive
mkdir -p "$ARCHIVE"

TO_ARCHIVE=$(cat <<EOF
patch-25.50a-fix1
patch-25.50a-v-ci-fix
patch-25.50a-w-tiered-ci-patch-audit
patch-25.50a-x-regression-matrix-fix
patch-25.50a-y-regression-matrix-eof-fix
patch-25.50a-yb-ci-trigger-optimization
patch-25.50a-z-auto-merge-enable
patch-25.50b-c-system-safeguard-suite
patch-25.50b-d-auto-merge-trigger-fix
patch-25.50b-e-ci-trigger-filter-fix
patch-25.60d-cleanup-workflow-automerge
patch-25.60f-automerge-fix
patch-25.60g-codex-auto-label
patch-25.65b-fix-enter
patch-25.65e-snapshot-save
patch-25.65f-add-dev-mode-
EOF
)

for patch in $TO_ARCHIVE; do
  if [ -d "patches/$patch" ]; then
    echo "📁 Archiving: $patch"
    mv "patches/$patch" "$ARCHIVE/"
  fi
done

echo "✅ Patch folder pruning complete."

