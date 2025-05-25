#!/bin/bash

Patch 25.51t-a Test Plan

1. Confirm: No trailing border at far right of BeamX box

./prismx --test beamx_no_right_overdraw

2. Confirm: Shimmer stops cleanly at edge with fade out

./prismx --test beamx_shimmer_fade

3. Confirm: Debug output prints shimmer boundaries

./prismx --test beamx_shimmer_debug_bounds

4. Confirm: BeamX respects layout region (no overflow)

./prismx --test beamx_render_bounds
