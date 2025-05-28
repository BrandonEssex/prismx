#!/bin/bash
set -e

echo "🧪 Patch 25.45g Test Plan: Layout Direction Fix"

grep -q "SIBLING_SPACING_X" src/gemx/layout.rs && echo "✅ Sibling spacing defined"
grep -q "CHILD_SPACING_Y" src/gemx/layout.rs && echo "✅ Child spacing defined"
grep -q "FREE_GRID_COLUMNS" src/gemx/layout.rs && echo "✅ Free node grid constant present"

echo "⚠️ Visually confirm siblings spread horizontally and free nodes fill the space"
echo "✅ Layout fix static checks complete"

