#!/bin/bash
set -e

echo "🧰 PrismX Codex Bootstrap Starting..."

# Step 1: Ensure Rust toolchain
if ! command -v cargo &> /dev/null; then
  echo "🦀 Installing Rust..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
else
  echo "✅ Rust already installed"
fi

# Step 2: System dependencies (Ubuntu only)
echo "📦 Installing libxcb (needed for render builds)..."
sudo apt-get update
sudo apt-get install -y libxcb-shape0-dev libxcb-xfixes0-dev

# Step 3: Build check
echo "🛠️ Building PrismX..."
cargo build --release

# Step 4: Run test plan (if it exists)
if [[ -f patches/patch-25.43-link-arrows-mac-scroll/test_plan.sh ]]; then
  chmod +x patches/patch-25.43-link-arrows-mac-scroll/test_plan.sh
  ./patches/patch-25.43-link-arrows-mac-scroll/test_plan.sh
fi

echo "✅ Codex Environment Ready!"
