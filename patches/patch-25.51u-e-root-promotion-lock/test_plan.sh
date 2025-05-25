#!/bin/bash

Patch 25.51u-e Test Plan

./prismx --test suppress_fallback_log_loop
./prismx --test promote_once_per_session
./prismx --test root_stable_after_promotion
./prismx --test prevent_repeat_promote_same_node
