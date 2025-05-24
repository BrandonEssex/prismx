#!/bin/zsh
set -e
echo "🧪 Patch 25.50a Test Plan: BeamX Logo Renderer"

grep -q "render_beam_logo" src/beamx.rs && echo "✅ render_beam_logo() defined"
echo "⚠️ On launch, confirm logo appears centered"
echo "⚠️ Confirm color lines form an X"
echo "⚠️ Optionally test shimmer / tick animation"
