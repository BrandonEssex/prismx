#!/bin/bash

Patch 25.51u-w Test Plan

./prismx --test simulate_enter_tab_sequence_inserts_correct_nodes
./prismx --test simulate_drill_pop_preserves_stack
./prismx --test simulate_delete_removes_node
./prismx --test simulate_logs_each_step_in_debug_mode
