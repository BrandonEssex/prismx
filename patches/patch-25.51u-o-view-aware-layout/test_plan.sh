#!/bin/bash
set -e

echo "🔍 Running Patch 25.51u-o Tests"

# Build and run unit tests
cargo test

echo "✅ Layout functions compile with view-aware clustering"
