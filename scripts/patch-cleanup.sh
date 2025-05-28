#!/bin/bash
set -euo pipefail

echo "📦 Scanning for invalid patch folders..."

# Find nested 'patches/patch-*' directories (accidental nesting)
find patches/patch-* -type d -name 'patch-*' | while read -r nested; do
  echo "❌ Nested patch folder: $nested"
  echo "→ Moving to backup: backup-patches/"
  mkdir -p backup-patches
  mv "$nested" backup-patches/
done

# Find empty patch directories
find patches -type d -name 'patch-*' -empty -print | while read -r empty; do
  echo "🗑 Removing empty patch folder: $empty"
  rmdir "$empty"
done

# Remove temp files and system artifacts
find patches -name '.DS_Store' -delete
find patches -name '*.swp' -delete
find patches -name '*~' -delete

echo "✅ Patch folder cleanup complete."
