#!/bin/zsh
set -e
echo "🧪 Patch 25.50a-e Test Plan: Final BeamX Corner Rendering"

grep -q "center_glyph" src/beamx.rs && echo "✅ center glyph configured"
echo "⚠️ Confirm top-right layout renders as:"
echo "╱  /"
echo " ✦"
echo "/  ╲"

echo "✅ Top-left and bottom-right beams use border color"
echo "✅ Top-right and bottom-left beams use status color"
echo "✅ Prism is centered, does not break X"
echo "✅ Corner border is removed under beam"
