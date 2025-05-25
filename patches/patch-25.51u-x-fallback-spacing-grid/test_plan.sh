#!/bin/bash

Patch 25.51u-x Test Plan

./prismx --test fallback_nodes_do_not_overlap
./prismx --test fallback_grid_placement_advances
./prismx --test grid_wraps_on_height_overflow
./prismx --test fallback_coords_remain_stable_between_frames
