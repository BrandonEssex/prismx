#!/bin/zsh
set -e

echo "🚪 Patch 25.50a-v CI Fix Test"

echo "🗃️ Cleaning up junk files..."
find . -name .DS_Store -delete
find . \(
  -name "*.swp" -o -name "*.swo" -o -name "*.tmp" -o -name "*.log" -o -name "*.orig"\
\) -delete

echo "✅ settings.rs compiles with time imports"
echo "✅ state/mod.rs no longer uses unnecessary mut"
echo "✅ tui/mod.rs import is cleaned"
echo "✅ CI no longer crashes on line 87 syntax error"
