#!/bin/bash
set -e

echo "🧪 Patch 25.51 Test Plan: Shortcut Manager + Debug Logger"

grep -q "debug_input_mode" src/state.rs && echo "✅ debug_input_mode exists"
grep -q "status_message" src/state.rs && echo "✅ status_message added"
grep -q "Ctrl.+Shift.+D" src/shortcuts.rs && echo "✅ ToggleDebugInput shortcut mapped"
grep -q "match_shortcut" src/shortcuts.rs && echo "✅ match_shortcut defined"

echo "⚠️ Manually confirm key events print to status bar, not terminal."
echo "✅ Patch 25.51 static checks complete"

