#!/bin/bash
set -e

echo "🧪 Patch 25.49 Test Plan: Link visuals and detach"

grep -q "╰─" src/screen/gemx.rs && echo "✅ elbow glyph updated"
grep -q "is_dragged" src/screen/gemx.rs && echo "✅ drag highlight logic"
grep -q "detach_node" src/gemx/interaction.rs && echo "✅ detach function present"

echo "✅ Patch 25.49 static checks complete"
