#!/bin/bash

Patch 25.51w Test Plan

./prismx --test drill_to_valid_node_focuses_correct_root
./prismx --test pop_returns_to_previous_root
./prismx --test drill_fails_gracefully_on_deleted_node
./prismx --test scroll_and_zoom_reset_on_pop
