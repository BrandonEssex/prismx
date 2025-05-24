#!/bin/zsh
set -e
echo "🧪 Patch 25.50a-c Test Plan: BeamX Module Frame + Logo"

grep -q "BeamStyle" src/beamx.rs && echo "✅ BeamStyle struct defined"
grep -q "style_for_mode" src/beamx.rs && echo "✅ Per-module styles defined"

echo "⚠️ Switch to each mode (gemx, zen, triage, settings)"
echo "✅ Confirm border and beams change color"
echo "✅ Confirm logo renders an X with ◉ at center"
echo "✅ Confirm border wraps entire frame"
