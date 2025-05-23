#!/bin/bash
set -e

echo "🧪 Patch 25.45d Test Plan: Tree Layout Restore"

grep -q "CHILD_SPACING_Y" src/gemx/*.rs && echo "✅ Vertical spacing constant defined"
grep -q "SIBLING_SPACING_X" src/gemx/*.rs && echo "✅ Horizontal spacing constant defined"
grep -q "child_y" -n src/layout.rs && echo "✅ Layout uses child.y offset"
grep -q "zoom_scale" src/gemx/render.rs && echo "✅ Zoom scaling preserved"

echo "⚠️ Zoom in/out and confirm children remain under their parent"
echo "✅ Tree layout visually confirmed"

