#!/bin/bash

Patch 25.51u-o Test Plan

./prismx --test fallback_node_is_rendered_immediately
./prismx --test node_role_set_root_after_fallback
./prismx --test no_infinite_fallback_loop
./prismx --test drawn_at_contains_fallback_node
