#!/bin/bash

Patch 25.54a Test Plan

./prismx --test settings_panel_renders_properly
./prismx --test toggle_switches_setting
./prismx --test settings_navigation_wraps_correctly
./prismx --test all_settings_apply_immediately
