# Patch 25.45k – Smart Grid Layout

## Goals
- Replace simple grid math with a scan that finds the first open cell
- Prevent overlap by checking occupied coordinates
- Use larger spacing so manual nodes don't collide
- Keep auto-arrange behaviour unchanged: on = dynamic layout, off = frozen positions
