use std::process::{Command, Stdio};
use std::time::Duration;
use std::io::{self, Read};
use std::thread;

pub fn run_plugin_command(entry: &str) -> io::Result<String> {
    let mut child = Command::new(entry)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No stdout"))?;

    let mut reader = std::io::BufReader::new(stdout);
    let mut output = String::new();
    reader.read_to_string(&mut output)?;

    let _ = child.wait_timeout(Duration::from_secs(10));

    Ok(output)
}
