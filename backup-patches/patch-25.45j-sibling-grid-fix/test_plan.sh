#!/bin/bash
set -e

echo "🧪 Patch 25.45j Test Plan: Sibling + Grid Fix"

grep -q "SIBLING_SPACING_X" src/layout.rs && echo "✅ SIBLING_SPACING_X present"
grep -q "CHILD_SPACING_Y" src/layout.rs && echo "✅ CHILD_SPACING_Y present"
grep -q "FREE_GRID_COLUMNS" src/layout.rs && echo "✅ FREE_GRID_COLUMNS defined"

echo "⚠️ Visually confirm no overlap of siblings or free nodes"
echo "✅ Confirm new free nodes spawn in open grid positions"

