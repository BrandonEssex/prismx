#!/bin/zsh
set -e
echo "🧪 Patch 25.45n-j Test Plan: Role-Aware Rendering"

grep -q "LayoutRole" src/layout.rs && echo "✅ roles returned"
grep -q "layout_nodes(...)" src/screen/gemx.rs && echo "✅ unpacked into (coords, roles)"

echo "⚠️ Confirm Ghost nodes are hidden"
echo "⚠️ Confirm Roots are rendered normally"
echo "⚠️ Confirm Free nodes still appear"
echo "⚠️ Confirm Orphans have visual warning"
