#!/bin/zsh
set -e
echo "🧪 Patch 25.46b Test Plan: Spotlight Match Sorting"

grep -q "score_match" src/spotlight.rs && echo "✅ scoring logic added"
echo "⚠️ Type part of a command, confirm fuzzy match ranks results"
echo "⚠️ Confirm grouped result sections: [Commands], [Tags], [Nodes]"
echo "⚠️ Top match is visually distinct"
