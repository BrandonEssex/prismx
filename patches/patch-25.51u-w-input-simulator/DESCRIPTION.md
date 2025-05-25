# Patch 25.51u-w – Input Simulation Framework

## Goals
- Allow simulation of key input sequences via `/simulate` command
- Support sequences like: `enter tab enter delete`
- Trigger input handlers as if keys were pressed
- Log after each step if debug mode is on
- Enable future patch regression testing and CI-style playback
