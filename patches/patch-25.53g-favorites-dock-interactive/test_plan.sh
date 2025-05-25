#!/bin/bash

Patch 25.53g Test Plan

./prismx --test dock_icon_click_opens_module
./prismx --test dock_bounds_match_visuals
./prismx --test mouse_click_outside_dock_does_nothing
./prismx --test plugin_entry_can_register_to_dock
