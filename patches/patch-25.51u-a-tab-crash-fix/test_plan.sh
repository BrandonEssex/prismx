#!/bin/bash

Patch 25.51u-a Test Plan

1. Test: Tab crash on empty root fallback

./prismx --test insert_child_tab_no_crash

2. Test: Ensure root promotion happens BEFORE layout_nodes()

./prismx --test pre_layout_root_check

3. Test: Avoid duplicate promotion log spam

./prismx --test dedup_promotion_logging

4. Test: Render returns early if root_nodes is still empty

./prismx --test empty_root_guard
