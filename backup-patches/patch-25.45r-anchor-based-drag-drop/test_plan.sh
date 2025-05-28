#!/bin/zsh
set -e
echo "🧪 Patch 25.45r Test Plan: Anchor-Based Drag Safety"
grep -q "complete_drag" src/gemx/interaction.rs && echo "✅ Drag logic entry point found"
echo "⚠️ Try dragging a node onto its child – must reject cycle"
echo "⚠️ Drop node on empty space – confirm new free node is placed"
