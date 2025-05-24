#!/bin/bash
set -e

echo "🧪 Patch 25.45k Test Plan: Smart Grid Layout"

grep -q "SIBLING_SPACING_X" src/layout.rs && echo "✅ SIBLING_SPACING_X present"
grep -q "CHILD_SPACING_Y" src/layout.rs && echo "✅ CHILD_SPACING_Y present"
grep -q "FREE_GRID_COLUMNS" src/layout.rs && echo "✅ FREE_GRID_COLUMNS defined"

echo "⚠️ Confirm free nodes spawn in visible empty locations"
echo "⚠️ Confirm spacing does not cause overlap"
echo "⚠️ Confirm turning OFF auto-arrange preserves layout"

