#!/bin/bash

Patch 25.51t-c Test Plan

./prismx --test right_border_has_gap_between_arrows
./prismx --test top_right_and_bottom_right_corners_visible
./prismx --test outbound_arrows_match_border_then_shimmer
./prismx --test inbound_arrows_allow_multicolor
./prismx --test beamx_visuals_dont_overlap_frame
