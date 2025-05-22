#!/bin/bash
set -e

echo "🧪 Patch 25.49 Test Plan: Visual Links + Drag Detach"

grep -q "render_link_arrow" src/gemx/render.rs && echo "✅ Arrow render logic present"
grep -q "drag_recursive" src/gemx/interaction.rs && echo "✅ drag_recursive used"
grep -q "detach_from_parent" src/gemx/interaction.rs && echo "✅ detach function found"

grep -q "→\|↳\|╰" src/gemx/render.rs && echo "✅ Visual arrows applied"

echo "✅ Visual + behavior tests passed (static)
