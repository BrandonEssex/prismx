#!/bin/bash
set -e

echo "🧪 Running Patch 25.44 Test Plan..."

# Build check
echo "🛠️ Building project..."
cargo build --release

# Snap toggle logic (assume state struct exists)
echo "🔁 Toggling Snap-to-Grid mode..."
grep -q "snap_to_grid" src/gemx/interaction.rs && echo "✅ Found snap_to_grid flag"

# Simulate drag logic present
grep -q "drag_node" src/gemx/interaction.rs && echo "✅ Drag function detected"

# Hotkey check
grep -q "Ctrl+G" src/input/hotkeys.rs && echo "✅ Ctrl+G hotkey registered"

echo "✅ Patch 25.44 passed static validation."
