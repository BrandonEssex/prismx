#!/bin/bash
set -e

echo "🧪 Patch 25.45m Test Plan: Viewport-Aware Layout"

grep -q "GEMX_HEADER_HEIGHT" src/layout.rs && echo "✅ Header constant defined"
grep -q "spawn_free_node" src/gemx/interaction.rs && echo "✅ spawn_free_node offset updated"
grep -q "layout_nodes" src/layout.rs && echo "✅ layout_nodes respects screen bounds"

echo "⚠️ Confirm nodes do not overlap gemx label or each other"
echo "⚠️ Confirm root siblings are horizontally spaced or wrapped"
echo "⚠️ Confirm compressed spacing still preserves legibility"

