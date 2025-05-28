#!/bin/bash
set -euo pipefail

echo "🔎 Scanning for orphan patch folders..."

ORPHAN_DIR="patches/_orphaned"
mkdir -p "$ORPHAN_DIR"

# Iterate over patch directories directly under patches/
shopt -s nullglob
for dir in patches/patch-*; do
  [ -d "$dir" ] || continue
  name=$(basename "$dir")

  if git log --oneline | grep -q "$name"; then
    echo "✅ $name – used"
  else
    echo "❌ $name – orphaned → moved to _orphaned"
    mv "$dir" "$ORPHAN_DIR/"
  fi
done
shopt -u nullglob

echo "✅ Orphan scan complete."
