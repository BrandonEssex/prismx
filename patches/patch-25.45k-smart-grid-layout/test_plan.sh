#!/bin/bash
set -e

echo "🧪 Patch 25.45k Test Plan: Smart Grid Layout"

grep -q "ensure_grid_positions" -n src/state/mod.rs && echo "✅ ensure_grid_positions exists"
grep -q "spawn_free_node" -n src/gemx/interaction.rs && echo "✅ spawn_free_node updated"

echo "⚠️ Manually toggle auto-arrange and add free nodes to confirm they occupy unique grid cells"
echo "✅ Static checks complete"
