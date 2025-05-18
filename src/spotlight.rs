use std::collections::HashMap;

pub fn use_command(input: &str) {
    let mut commands: HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    commands.insert("/theme dark", Box::new(|| {
        println!("[SPOTLIGHT] Theme set to dark");
    }));

    commands.insert("/plugin disable mindtrace", Box::new(|| {
        println!("[SPOTLIGHT] mindtrace plugin disabled");
    }));

    commands.insert("/journal", Box::new(|| {
        crate::zen::start_journal().unwrap();
    }));

    commands.insert("/triage", Box::new(|| {
        println!("[SPOTLIGHT] Triage toggled.");
    }));

    commands.insert("/copy", Box::new(|| {
        crate::clipboard::copy_node("example");
    }));

    match commands.get(input.trim()) {
        Some(action) => action(),
        None => println!("[SPOTLIGHT] Unknown command: {}", input),
    }
}
