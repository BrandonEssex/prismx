#!/bin/zsh
set -e
echo "🧪 Patch 25.50a-g Test Plan: BeamX Final Edge Integration"

grep -q "render_beam_logo" src/screen/*.rs && echo "✅ Beam render present"

echo "⚠️ Confirm:"
echo "✅ Top-right corner is fully replaced by beams"
echo "✅ Prism logo is centered (◆ preferred)"
echo "✅ Border does NOT draw overlapping glyphs (┓, ━, ┃)"
echo "✅ Ctrl+. opens Settings"
