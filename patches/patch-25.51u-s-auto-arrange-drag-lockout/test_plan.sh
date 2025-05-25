#!/bin/bash

Patch 25.51u-s Test Plan

# Ensure drag handler skips when auto-arrange is active
grep -q "Drag disabled while auto-arrange is enabled" src/gemx/interaction.rs && echo "✅ status message added"
grep -q "auto_arrange" -n src/gemx/interaction.rs | head
