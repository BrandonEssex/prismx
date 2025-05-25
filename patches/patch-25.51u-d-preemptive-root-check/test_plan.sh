#!/bin/bash

Patch 25.51u-d Test Plan

./prismx --test tab_insert_no_crash_pre_root
./prismx --test fallback_executes_before_layout
./prismx --test root_injection_prior_to_role_calc
./prismx --test layout_safe_after_tab_press
