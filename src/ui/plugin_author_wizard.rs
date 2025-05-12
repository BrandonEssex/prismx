use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub fn scaffold_plugin(id: &str, entry: &str) -> std::io::Result<()> {
    let plugin_dir = format!("{}/.prismx/plugins/{}", std::env::var("HOME").unwrap(), id);
    let manifest_path = format!("{}/manifest.toml", plugin_dir);
    let entry_path = format!("{}/{}", plugin_dir, entry);

    create_dir_all(&plugin_dir)?;

    let mut manifest = File::create(manifest_path)?;
    writeln!(manifest, "[plugin]")?;
    writeln!(manifest, "name = \"{}\"", id)?;
    writeln!(manifest, "id = \"{}\"", id)?;
    writeln!(manifest, "enabled = true")?;
    writeln!(manifest, "entry = \"{}\"", entry)?;
    writeln!(manifest, "launcher_keywords = [\"{}\"]", id)?;
    writeln!(manifest, "[ui]")?;
    writeln!(manifest, "tabbed = true")?;
    writeln!(manifest, "panel = \"right\"")?;

    let mut script = File::create(entry_path)?;
    writeln!(script, "#!/bin/bash")?;
    writeln!(script, "echo \"{} plugin launched!\"", id)?;

    Ok(())
}
