pub struct Profiler;

impl Profiler {
    pub fn profile(&self, plugin_name: &str) {
        log::info!("Profiling plugin: {}", plugin_name);
    }
}
