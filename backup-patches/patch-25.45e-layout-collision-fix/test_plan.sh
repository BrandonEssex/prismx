#!/bin/bash
set -e

echo "🧪 Patch 25.45e Test Plan: Layout Collision Fix"

grep -q "SIBLING_SPACING_X" src/gemx/layout.rs && echo "✅ Sibling spacing constant present"
grep -q "CHILD_SPACING_Y" src/gemx/layout.rs && echo "✅ Child spacing constant present"
grep -q "child.x = parent.x" src/gemx/layout.rs && echo "✅ Child inherits parent x"
grep -q "child.y = parent.y + CHILD_SPACING_Y" src/gemx/layout.rs && echo "✅ Vertical offset applied"

echo "⚠️ Manually add multiple siblings and children, then zoom in/out to verify spacing"
echo "✅ Static test complete"

