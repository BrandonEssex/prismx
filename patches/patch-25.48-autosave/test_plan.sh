#!/bin/bash
set -e

echo "🧪 Patch 25.48 Test Plan: Autosave"

grep -q "autosave" src/state/*.rs && echo "✅ autosave module found"
grep -q "thread::sleep" src/state/autosave.rs && echo "✅ autosave loop present"
grep -q "dirty" src/state/*.rs && echo "✅ dirty flag used"

grep -q "autosave.json" src/storage/*.rs && echo "✅ autosave file path defined"

echo "✅ Static autosave checks complete"
