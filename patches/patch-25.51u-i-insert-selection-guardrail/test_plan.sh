#!/bin/bash

Patch 25.51u-i Test Plan

./prismx --test enter_updates_selection
./prismx --test tab_requires_valid_selection
./prismx --test no_tab_fallback_if_selection_is_valid
./prismx --test child_is_selected_after_tab
./prismx --test sibling_is_selected_after_enter
