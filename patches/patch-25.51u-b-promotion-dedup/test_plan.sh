#!/bin/bash

Patch 25.51u-b Test Plan

1. Run render loop with fallback trigger

./prismx --test fallback_log_deduplication

2. Confirm only ONE "promoted Node" message appears

./prismx --test no_log_spam_on_recovery

3. Node remains in root_nodes only once

./prismx --test root_node_deduplication

4. Confirm previous behavior remains unchanged for Tab/Enter

./prismx --test insert_tab_stable
./prismx --test insert_enter_stable
