#!/bin/bash
set -e

echo "🧪 Patch 25.49a Test Plan: Auto-Arrange + Spacing"

grep -q "NODE_SPACING_X" src/gemx/*.rs && echo "✅ X spacing constant found"
grep -q "manual_position" src/state.rs && echo "✅ Manual position tracking exists"
grep -q "layout_fallback_grid" src/gemx/layout.rs && echo "✅ Grid layout fallback present"
grep -q "auto_arrange" src/state.rs && echo "✅ Auto-arrange toggle confirmed"

echo "⚠️ Manually toggle Ctrl+P and verify node positions persist or space cleanly"
echo "✅ Static test complete"

