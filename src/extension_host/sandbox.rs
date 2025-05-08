pub struct Sandbox;

impl Sandbox {
    pub fn execute(&self, code: &str) -> Result<String, String> {
        log::info!("Executing plugin code (simulated)");
        Ok(format!("Executed: {}", code))
    }
}
