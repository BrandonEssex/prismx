# Patch 25.51u-o – Emergency Draw Injection + Fallback Clamp

## Goals
- Forcefully inject fallback-promoted root node into the render map
- Assign layout role explicitly on fallback
- Prevent infinite fallback retry
- Guarantee visual appearance of fallback node
