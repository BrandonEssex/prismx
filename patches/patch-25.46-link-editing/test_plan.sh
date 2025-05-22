#!/bin/bash
set -e

echo "🧪 Patch 25.46 Test Plan: Link Editing"

grep -q "Ctrl.+Backspace" src/input/hotkeys.rs && echo "✅ Delete hotkey mapped"
grep -q "Ctrl.+Shift.+L" src/input/hotkeys.rs && echo "✅ Link swap hotkey mapped"

grep -q "remove_link" src/gemx/*.rs && echo "✅ remove_link function present"
grep -q "reconnect_link" src/gemx/*.rs && echo "✅ reconnect_link function present"

echo "✅ Static link-edit checks complete"
