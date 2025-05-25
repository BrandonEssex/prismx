#!/bin/bash

Patch 25.51u-q Test Plan

./prismx --test insert_logs_coordinates_in_debug_mode
./prismx --test root_node_gets_manual_position
./prismx --test sibling_does_not_overlap_in_manual_mode
./prismx --test render_tree_shows_correct_node_positions
