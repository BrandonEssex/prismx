#!/bin/bash

Patch 25.51u-v Test Plan

./prismx --test fallback_promoted_node_has_coords
./prismx --test fallback_node_has_role_root
./prismx --test fallback_draws_node_into_drawn_at
./prismx --test no_root_node_remains_invisible
