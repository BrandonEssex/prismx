#!/bin/bash

Patch 25.53c Test Plan

./prismx --test autocomplete_matches_partial_command
./prismx --test autocomplete_updates_dynamically
./prismx --test autocomplete_render_position_is_correct
