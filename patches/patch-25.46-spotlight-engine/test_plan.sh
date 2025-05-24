#!/bin/zsh
set -e

echo "🧪 Patch 25.46 Test Plan: Spotlight Engine"

grep -q "Spotlight" src/spotlight.rs && echo "✅ spotlight.rs defined"
grep -q "show_spotlight" src/state/mod.rs && echo "✅ spotlight state present"

echo "⚠️ Alt+Space opens Spotlight"
echo "⚠️ Typing filters results"
echo "⚠️ Enter triggers action"
echo "⚠️ ESC exits and restores focus"
