#!/bin/bash
set -e

# Patch 25.62c Test Plan
# 1. Drop a PNG file onto the terminal. Verify entry appears with thumbnail.
# 2. Paste an image from clipboard. Confirm attachment entry with caption prompt.
# 3. Toggle media_mode in settings.toml off and ensure drops are ignored.
