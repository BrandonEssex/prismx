#!/bin/bash

Patch 25.51u-s Test Plan

./prismx --test debug_trace_on_every_insert
./prismx --test unreachable_node_gets_fallback_coords
./prismx --test drawn_at_logs_all_visible_nodes
./prismx --test no insert goes unlogged in debug mode
