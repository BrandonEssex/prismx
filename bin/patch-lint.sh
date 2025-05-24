#!/bin/bash
set -e

patch_dir="\$1"
[ -z "\$patch_dir" ] && echo "Usage: bin/patch-lint.sh patch-folder-name" && exit 1

cd patches/\$patch_dir || exit 1

# Validate required files
for f in DESCRIPTION.md CODE_CHANGES.md test_plan.sh; do
  [ ! -f "\$f" ] && echo "❌ Missing \$f in \$patch_dir" && exit 1
done

# Validate headers
grep -q '^# Patch ' DESCRIPTION.md || { echo "❌ DESCRIPTION.md missing title"; exit 1; }
grep -q '^## Code Changes' CODE_CHANGES.md || { echo "❌ CODE_CHANGES.md missing section"; exit 1; }

echo "✅ Patch lint passed for \$patch_dir"
