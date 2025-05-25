#!/bin/bash

Patch 25.51u-c Test Plan

./prismx --test root_promotion_once
./prismx --test no_log_spam_without_debug
./prismx --test unreachable_nodes_promoted_once
./prismx --test render_dedup_root_nodes
