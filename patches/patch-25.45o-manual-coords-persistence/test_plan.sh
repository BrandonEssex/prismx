#!/bin/zsh
set -e
echo "🧪 Patch 25.45o Test Plan: Manual Coords Persistence"
grep -q "manual_coords" src/node.rs && echo "✅ manual_coords field defined"
echo "⚠️ Drag node with auto_arrange OFF, confirm position is saved"
