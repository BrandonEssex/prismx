#!/bin/zsh
set -e
echo "🧪 Patch 25.45p-a Test Plan: NodeType Assignment + Labeling"

grep -q "NodeType" src/node.rs && echo "✅ NodeType enum defined"
grep -q "node_type" src/node.rs && echo "✅ NodeType field added to Node"

echo "⚠️ Add new nodes, confirm default type is Text"
echo "⚠️ Assign type: Link → confirm 📎 drawn"
echo "⚠️ Assign type: Task → confirm ☑ drawn"
