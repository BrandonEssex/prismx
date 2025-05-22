#!/bin/bash
set -e

echo "🔧 Running PrismX Codex Setup..."

# Move to actual project directory
cd /workspace/prismx

# Check that Rust is available
if ! command -v cargo &> /dev/null; then
  echo "❌ Rust toolchain not found. Cannot continue."
  exit 127
fi

# Build the project once (with network access)
echo "🛠️ Building PrismX..."
cargo build --release

# Ensure the offline tester is ready
echo "🔐 Setting up offline tools..."
chmod +x /workspace/prismx/bin/offline-patch-check.sh
chmod +x /workspace/prismx/bin/reset-codex-env.sh
chmod +x /workspace/prismx/bin/commit-patch.sh

echo "✅ PrismX environment is ready for offline patching and testing."
