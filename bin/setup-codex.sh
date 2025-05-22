#!/bin/bash
set -e

echo "🔧 PrismX Setup Starting..."

# Skip curl/rustup if Rust is already pre-installed
if ! command -v cargo &> /dev/null; then
  echo "❌ Rust toolchain not found. Please preinstall Rust in this environment."
  exit 127
fi

# Build project
echo "🛠️ Running cargo build..."
cargo build --release

# Run test plan if present
if [[ -x patches/patch-25.44-drag-drop-snap/test_plan.sh ]]; then
  ./patches/patch-25.44-drag-drop-snap/test_plan.sh
else
  echo "⚠️ No test plan found for current patch."
fi

echo "✅ Setup complete."
