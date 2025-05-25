#!/bin/bash

Patch 25.51u-y Test Plan

./prismx --test add_child_prevents_self_link
./prismx --test layout_recursion_clamps_properly
./prismx --test child_insert_positions_correctly_when_manual
./prismx --test debug_logs_on_recursion_exit
