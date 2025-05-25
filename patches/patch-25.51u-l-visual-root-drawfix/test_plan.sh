#!/bin/bash

Patch 25.51u-l Test Plan

./prismx --test root_node_with_no_children_is_drawn
./prismx --test sibling_insert_in_root_context_does_not_break_layout
./prismx --test fallback_promoted_node_remains_visible
./prismx --test layout_recovers_after_tab_enter_tab
