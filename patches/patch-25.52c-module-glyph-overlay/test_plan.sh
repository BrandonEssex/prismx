#!/bin/bash

Patch 25.52c Test Plan

./prismx --test module_icon_visible_in_each_mode
./prismx --test icon_position_does_not_overlap_ui
./prismx --test switching_module_updates_glyph
./prismx --test icon_remains_in_corner_during_interaction
