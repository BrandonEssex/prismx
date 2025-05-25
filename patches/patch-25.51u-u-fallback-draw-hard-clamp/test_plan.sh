#!/bin/bash

Patch 25.51u-u Test Plan

./prismx --test fallback_nodes_assigned_safe_coords
./prismx --test fallback_drawn_at_injected_with_guarded_rect
./prismx --test no_render_overflows_terminal_bounds
./prismx --test no repeating fallback log or layout crash
