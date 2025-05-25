#!/bin/bash

Patch 25.51u-f Test Plan

./prismx --test unreachable_nodes_promoted_once_per_frame
./prismx --test fallback_skips_nodes_without_children
./prismx --test fallback_resets_each_render
./prismx --test stable_root_state_after_multiple_frames
