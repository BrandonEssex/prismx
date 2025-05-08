use super::sandbox::Sandbox;

pub struct SandboxHost {
    sandbox: Sandbox,
}

impl SandboxHost {
    pub fn new() -> Self {
        Self {
            sandbox: Sandbox,
        }
    }

    pub fn run_plugin(&self, code: &str) -> Result<String, String> {
        self.sandbox.execute(code)
    }
}
