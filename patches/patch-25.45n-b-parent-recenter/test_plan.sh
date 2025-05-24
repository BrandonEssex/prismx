#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-b Test Plan: Recenter Parent"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout logic patch point located"

echo "⚠️ Add long branches to one child and confirm parent recenters horizontally"
echo "⚠️ Confirm spacing remains symmetric for uneven children"

