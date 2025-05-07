#!/bin/bash

# PrismX Startup Script
# Cleans, recompiles, and runs PrismX explicitly.

# Exit immediately if a command exits with a non-zero status.
set -e

# Step 1: Clean Cargo build artifacts
echo "🚧 Running cargo clean..."
cargo clean

# Step 2: Remove additional temporary/unnecessary files (customize as needed)
echo "🧹 Cleaning additional temporary files..."
# Example: Remove temporary logs, sessions, or swap files (customize paths as necessary)
rm -rf ./target/debug
rm -rf ./target/release
rm -rf ./logs/*
rm -rf ./tmp/*
find . -name "*.log" -type f -delete
find . -name "*.tmp" -type f -delete

# (Add or remove cleanup commands explicitly as required)

# Step 3: Compile the project in release mode
echo "⚙️ Compiling PrismX in release mode..."
cargo build --release

# Step 4: Run the PrismX application
echo "🚀 Launching PrismX..."
RUST_LOG=info ./target/release/prismx

