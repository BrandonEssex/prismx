#!/bin/bash
set -e

echo "📄 Generating CHEATSHEET.md from src/ui/shortcuts.rs..."

SRC=src/ui/shortcuts.rs
DST=CHEATSHEET.md

if [ ! -f "$SRC" ]; then
  echo "❌ Could not find $SRC"
  exit 1
fi

echo "# PrismX Keyboard Shortcuts" > $DST
echo >> $DST

grep '("' "$SRC" | sed -E 's/ *\("([^"]+)", +"([^"]+)"\),?/` \1 ` → \2/' >> $DST

echo >> $DST
echo "_This file is generated from the SHORTCUTS constant. Edit `src/ui/shortcuts.rs` to make changes._" >> $DST

echo "✅ CHEATSHEET.md updated."
