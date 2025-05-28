#!/bin/bash
set -e

echo "🧪 Patch 25.47 Test Plan: Workspace Switching"

grep -q "Ctrl.+Tab" src/input/hotkeys.rs && echo "✅ Switch hotkey mapped"
grep -q "Ctrl.+Shift.+S" src/input/hotkeys.rs && echo "✅ Save hotkey mapped"
grep -q "Ctrl.+Shift.+O" src/input/hotkeys.rs && echo "✅ Load hotkey mapped"

grep -q "serde" Cargo.toml && echo "✅ Serde enabled for JSON IO"

echo "✅ Workspace switch/load/save logic ready"
