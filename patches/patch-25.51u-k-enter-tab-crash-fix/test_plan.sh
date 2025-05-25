#!/bin/bash

Patch 25.51u-k Test Plan

./prismx --test enter_tab_no_fallback
./prismx --test inserted_node_selected_correctly
./prismx --test tab_promotes_parent_if_needed
./prismx --test no_layout_wipe_after_tab
