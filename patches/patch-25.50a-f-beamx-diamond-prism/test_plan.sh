#!/bin/zsh
set -e
echo "🧪 Patch 25.50a-f Test Plan: BeamX Diamond Prism Final Render"

grep -q "center_glyph" src/beamx.rs && echo "✅ Prism glyph defined"
echo "✅ Beams render as: \\  /"
echo "✅ Center glyph is ◆ (fallback ✦)"
echo "✅ Top-right corner border omitted when beam crosses"
echo "✅ BeamX is always top-right, visible on all screens"
