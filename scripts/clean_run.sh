#!/bin/bash
set -e

echo "🧼 Checking Git status..."
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "⚠️ You have uncommitted changes. Commit or stash before running."
  exit 1
fi

echo "🚧 Running cargo clean..."
cargo clean

echo "📡 Syncing Git..."
git pull && git push

echo "🧹 Removing build artifacts and temp files..."
rm -rf ./target/debug ./target/release ./tmp/*
find . -name "*.log" -type f -delete
find . -name "*.tmp" -type f -delete
mkdir -p logs

echo "⚙️ Building PrismX (release)..."
cargo build --release | tee logs/build.log

if [ ! -f ./target/release/prismx ]; then
  echo "❌ Build failed — binary not found!"
  exit 1
fi

echo "🚀 Launching PrismX..."
RUST_LOG=info ./target/release/prismx
