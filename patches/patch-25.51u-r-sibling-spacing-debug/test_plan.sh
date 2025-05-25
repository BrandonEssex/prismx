#!/bin/bash

Patch 25.51u-r Test Plan

./prismx --test sibling_spacing_enforced_in_manual_layout
./prismx --test siblings_have_unique_x
./prismx --test fallback_sibling_clamped_if_at_x0
./prismx --test sibling_coords_logged_on_insert
