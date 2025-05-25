#!/bin/bash

Patch 25.51u-n Test Plan

./prismx --test root_drawn_even_without_children
./prismx --test fallback_does_not_trigger_on_rooted_node
./prismx --test enter_tab_enter_is_stable
./prismx --test layout_roles_assign_root_on_depth_zero
./prismx --test layout_render_visible_after fallback
