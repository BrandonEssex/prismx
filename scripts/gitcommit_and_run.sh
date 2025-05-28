#!/bin/bash

# Git + Build Runner for PrismX
# Author: Brandon Essex
# Version: 1.0.0
# License: MIT

set -euo pipefail

# Ensure a commit message is passed
if [ $# -eq 0 ]; then
  echo "❌ Error: Commit message required."
  echo "Usage: ./gitcommit.sh \"v2.4.0-beta5: RefractPack-v1.0.0\""
  exit 1
fi

COMMIT_MSG="$1"

# Step 1: Git Operations
echo "🔃 Staging and committing changes..."
git add .
git commit -m "$COMMIT_MSG"
git push

# Step 2: Clean build environment
echo "🚧 Running cargo clean..."
cargo clean

echo "🧹 Removing build artifacts and temporary files..."
rm -rf ./target/debug ./target/release ./logs/* ./tmp/*
find . -name "*.log" -type f -delete
find . -name "*.tmp" -type f -delete

# Step 3: Compile in release mode
echo "⚙️ Compiling PrismX in release mode..."
cargo build --release

# Step 4: Run PrismX
echo "🚀 Launching PrismX..."
RUST_LOG=info ./target/release/prismx
