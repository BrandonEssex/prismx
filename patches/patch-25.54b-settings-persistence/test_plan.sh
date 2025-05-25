#!/bin/bash

Patch 25.54b Test Plan

./prismx --test toggles_apply_on_load
./prismx --test settings_saved_to_toml
./prismx --test dock_layout_restores_correctly
./prismx --test debug_mode_persists_between_runs
