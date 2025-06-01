#!/bin/bash
set -e

# Simulated heartbeat verification

tmp=$(mktemp)
for i in {1..3}; do
  echo "HEARTBEAT_TICK $i" >> "$tmp"
  sleep 0.1
done

ticks=$(grep -c 'HEARTBEAT_TICK' "$tmp")
if [ "$ticks" -ge 2 ]; then
  echo "HEARTBEAT_OK"
  exit 0
else
  echo "HEARTBEAT_FAIL"
  exit 1
fi
