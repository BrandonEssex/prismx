#[derive(Debug, Clone)]
pub struct SpotlightModule;

impl SpotlightModule {
    pub fn new() -> Self {
        SpotlightModule
    }

    pub fn run(&self) {
        log::info!("Running spotlight search...");
    }
}