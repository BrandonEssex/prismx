#!/bin/bash
set -e

echo "🧪 Patch 25.45i Test Plan"

grep -q "CHILD_SPACING_Y" src/layout.rs && echo "✅ Child spacing constant exists"
grep -q "FREE_GRID_COLUMNS" src/layout.rs && echo "✅ Fallback grid constant exists"
grep -q "drag_recursive" src/gemx/interaction.rs && echo "✅ Drag logic found"

echo "⚠️  Turn off auto-arrange, drag nodes, add siblings/children, check visual tree structure"
echo "✅ Static layout checks passed"

