# Patch 25.50b-a – Plugin System Bootstrap

## Goals
- Add core plugin system (`plugins/mod.rs`)
- Create initial plugin modules: Countdown, Pomodoro
- Define `PluginRender` trait
- Wire into `RoutineForge` for manual test rendering
- Prepare Spotlight commands: `/start pomodoro`, `/add countdown`
