## Code Changes

- Adds basic plugin loader using `libloading::Library`
- Defines `PluginEntry` trait with lifecycle stub
- Logs available .so/.dylib modules from `plugins/`
