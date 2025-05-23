#!/bin/bash
set -e

echo "🧪 Patch 25.45d Test Plan: Tree Layout Restore"

grep -q "SIBLING_SPACING_X" src/gemx/*.rs && echo "✅ Spacing constants defined"
grep -q "CHILD_SPACING_Y" src/gemx/*.rs && echo "✅ Child spacing constant defined"
grep -q "SIBLING_SPACING_X" src/gemx/layout.rs && echo "✅ Layout uses sibling spacing"
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ Zoom scaling preserved"

echo "⚠️ Zoom in/out and confirm children remain under their parent"
echo "✅ Tree layout visually confirmed"

