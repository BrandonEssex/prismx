#!/bin/bash
set -e

echo "🧪 Patch 25.45f Test Plan: Void-Style Layout"

grep -q "SIBLING_SPACING_X" src/gemx/*.rs && echo "✅ Horizontal spacing constant defined"
grep -q "CHILD_SPACING_Y" src/gemx/*.rs && echo "✅ Vertical spacing constant defined"
grep -q "child.x = parent.x" src/gemx/layout.rs && echo "✅ Tree alignment logic present"

echo "⚠️ Zoom in/out, add 3+ siblings and 2+ children, and visually confirm structure"
echo "✅ Static checks complete"

