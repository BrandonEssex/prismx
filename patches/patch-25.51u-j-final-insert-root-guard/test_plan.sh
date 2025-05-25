#!/bin/bash

Patch 25.51u-j Test Plan

./prismx --test enter_always_selects_valid_node
./prismx --test tab_only_runs_with_valid_selection
./prismx --test tab_promotes_parent_if_needed
./prismx --test layout_does_not_wipe_after_tab
./prismx --test fallback_does_not_run_more_than_once
./prismx --test all_root_nodes_are_unique
