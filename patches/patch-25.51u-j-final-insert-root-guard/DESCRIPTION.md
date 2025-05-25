# Patch 25.51u-j – Final Insert + Root Recovery Fix

## Goals
- Ensure selection is always valid after insert
- Prevent `Tab` from orphaning children
- Automatically promote parents or new nodes if they would cause a layout wipe
- Block fallback loops and enforce layout visibility sanity
