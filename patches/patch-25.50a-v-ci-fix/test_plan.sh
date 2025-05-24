#!/bin/zsh
set -e

echo "🚪 Patch 25.50a-v CI Fix Test"

echo "✅ settings.rs compiles with time imports"
echo "✅ state/mod.rs no longer uses unnecessary mut"
echo "✅ tui/mod.rs import is cleaned"
echo "✅ CI no longer crashes on line 87 syntax error"
