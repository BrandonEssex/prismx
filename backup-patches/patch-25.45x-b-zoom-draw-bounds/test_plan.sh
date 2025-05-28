#!/bin/zsh
set -e
echo "🧪 Patch 25.45x-b Test Plan: Zoom Crash Fix"

grep -q "render_gemx" src/screen/gemx.rs && echo "✅ render_gemx patch point located"

echo "⚠️ Zoom in on a wide tree"
echo "⚠️ Confirm no crash occurs"
echo "⚠️ Confirm no arrows or nodes are drawn past right/bottom edge"

