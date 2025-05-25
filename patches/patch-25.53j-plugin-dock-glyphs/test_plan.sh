#!/bin/bash

Patch 25.53j Test Plan

./prismx --test plugin_registers_favorite_successfully
./prismx --test dock_renders_plugin_glyph
./prismx --test plugin_glyph_triggered_via_command
./prismx --test dock_caps_total_slots_to_5
