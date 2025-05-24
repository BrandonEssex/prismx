#!/bin/bash
set -e

echo "🧪 Patch 25.45l Test Plan: True Sibling Layout"

grep -q "layout_recursive_safe" src/layout.rs && echo "✅ layout_recursive_safe updated"
grep -q "SIBLING_SPACING_X" src/layout.rs && echo "✅ spacing constant present"

echo "⚠️ Confirm siblings center around parent"
echo "⚠️ Confirm no overlap of siblings"
echo "⚠️ Confirm children stack vertically under each sibling"

