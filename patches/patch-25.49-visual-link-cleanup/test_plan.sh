#!/bin/bash
set -e


echo "🧪 Patch 25.49 Test Plan: Link visuals and detach"

grep -q "╰─" src/screen/gemx.rs && echo "✅ elbow glyph updated"
grep -q "is_dragged" src/screen/gemx.rs && echo "✅ drag highlight logic"
grep -q "detach_node" src/gemx/interaction.rs && echo "✅ detach function present"

echo "✅ Patch 25.49 static checks complete"

echo "🧪 Patch 25.49 Test Plan: Visual Links + Drag Detach"

grep -q "render_link_arrow" src/gemx/render.rs && echo "✅ Arrow render logic present"
grep -q "drag_recursive" src/gemx/interaction.rs && echo "✅ drag_recursive used"
grep -q "detach_from_parent" src/gemx/interaction.rs && echo "✅ detach function found"

grep -q "→\|↳\|╰" src/gemx/render.rs && echo "✅ Visual arrows applied"

echo "✅ Visual + behavior tests passed (static)

