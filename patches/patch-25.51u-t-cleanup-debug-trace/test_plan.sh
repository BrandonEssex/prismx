#!/bin/bash

Patch 25.51u-t Test Plan

./prismx --test debug_output_is_clean
./prismx --test fallback_labels_are_removed
./prismx --test layout_logs_summary_only
./prismx --test snapshot_ready_layout_trace_clean
