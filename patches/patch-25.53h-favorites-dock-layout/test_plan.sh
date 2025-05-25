#!/bin/bash

Patch 25.53h Test Plan

./prismx --test dock_supports_horizontal_layout
./prismx --test dock_respects_limit_setting
./prismx --test dock_border_color_matches_theme
./prismx --test dock_renders_all_modes
./prismx --test dock_layout_toggle_via_command
