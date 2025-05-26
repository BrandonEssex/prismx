#!/bin/bash
set -e

patch="$1"
[ -z "$patch" ] && { echo "Usage: $0 <patch_dir>" >&2; exit 1; }

# allow calling with or without patches/ prefix
if [ -d "$patch" ]; then
  dir="$patch"
else
  dir="patches/$patch"
fi

required=("DESCRIPTION.md" "FILES_CHANGED.txt" "CODE_CHANGES.md")
for f in "${required[@]}"; do
  if [ ! -f "$dir/$f" ]; then
    echo "⚠️  Missing $f in $patch" >&2
    exit 1
  fi
done

exit 0
