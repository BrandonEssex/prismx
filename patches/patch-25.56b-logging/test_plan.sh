#!/bin/bash
# Patch 25.56b Test Plan

# 1. Launch PrismX with debug mode on
# 2. Trigger spotlight commands (e.g. /zen, /undo)
# 3. Validate eprintln! lines show `[DEBUG]` prefix
# 4. Disable debug mode – logs no longer print
