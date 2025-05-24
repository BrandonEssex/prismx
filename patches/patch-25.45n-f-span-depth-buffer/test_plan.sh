#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-f Test Plan: Sibling Depth Clamp"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic patch point located"

echo "⚠️ Add multiple child levels with narrow labels"
echo "⚠️ Confirm spacing remains even across wide trees"
echo "⚠️ Confirm no overlap on deep+wide siblings"

