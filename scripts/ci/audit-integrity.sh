#!/bin/bash
echo "🔍 Auditing PrismX Patch Integration..."

# Check logging
grep -q "init_logger()" src/main.rs || echo "❌ logging::init_logger() not called"

# Check Zen input toggle
grep -q "toggle_zen_view" src/zen/editor.rs || echo "❌ toggle_zen_view missing in editor.rs"

# Check plugin loader
grep -q "load_plugins" src/state/init.rs || echo "❌ Plugin loader not wired"

# Check drag-drop image support
grep -q "JournalEntry::Image" src/zen/editor.rs || echo "⚠️ image drop support not detected"

echo "✅ Audit complete"
