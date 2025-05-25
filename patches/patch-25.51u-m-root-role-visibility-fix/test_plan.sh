#!/bin/bash

Patch 25.51u-m Test Plan

./prismx --test fallback_root_nodes_drawn_always
./prismx --test unreachable_nodes_not_promoted_twice
./prismx --test orphan_root_role_is_assigned
./prismx --test layout_stable_after_enter_tab_enter
