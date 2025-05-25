#!/bin/bash

Patch 25.51u-p Test Plan

./prismx --test fallback_label_is_safe
./prismx --test fallback_node_always_draws
./prismx --test label_is_not_prefixed_repeatedly
./prismx --test layout_remains_stable_after_fallback
./prismx --test node_label_does_not_overflow_ui
