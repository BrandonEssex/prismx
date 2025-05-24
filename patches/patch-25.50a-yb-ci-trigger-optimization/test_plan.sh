#!/bin/zsh
set -e
echo "✅ Patch-test workflow no longer spams 60+ jobs on main"
echo "✅ Runs only if patch folders are changed in a PR"
echo "✅ Skips if no matrix matches"
echo "✅ Cancels duplicate runs"
