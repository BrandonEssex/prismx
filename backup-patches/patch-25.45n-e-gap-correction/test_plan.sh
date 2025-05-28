#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-e Test Plan: Label-Aware Spacing"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic patched"

echo "⚠️ Add children with long/short labels"
echo "⚠️ Confirm all siblings are spaced cleanly with at least 3 chars"
echo "⚠️ Check that parent is still centered over group"

