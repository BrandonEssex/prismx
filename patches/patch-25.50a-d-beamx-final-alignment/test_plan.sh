#!/bin/zsh
set -e
echo "🧪 Patch 25.50a-d Test Plan: BeamX Final Alignment"

grep -q "render_beam_logo" src/beamx.rs && echo "✅ logo render exists"

echo "⚠️ Launch GemX, Zen, Triage, Settings"
echo "✅ Confirm BeamX is top-right"
echo "✅ Confirm logo looks like:"
echo "\\  /"
echo "  *"
echo "/  \\"
echo "✅ Prism uses correct module-based color"
echo "✅ Logo stays visible during Help overlay"
