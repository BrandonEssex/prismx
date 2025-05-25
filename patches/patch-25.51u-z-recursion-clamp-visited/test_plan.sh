#!/bin/bash

Patch 25.51u-z Test Plan

./prismx --test deep_tree_layout_completes_without_stack_overflow
./prismx --test cycle_is_detected_and_skipped
./prismx --test max_depth_clamp_triggers_at_safe_limit
./prismx --test debug_log_outputs_skipped_node
