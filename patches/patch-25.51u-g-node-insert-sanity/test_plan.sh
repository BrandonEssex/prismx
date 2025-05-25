#!/bin/bash

Patch 25.51u-g Test Plan

./prismx --test tab_requires_valid_selection
./prismx --test enter_assigns_correct_parent
./prismx --test fallback_promotes_only_once_per_node
./prismx --test no_node_overlap_when_auto_arrange_off
./prismx --test orphans_get_layoutrole_free
