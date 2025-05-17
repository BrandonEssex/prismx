use std::collections::HashMap;

pub fn launch_spotlight() {
    println!("[SPOTLIGHT] Command bar active. Try:");
    println!("  /theme dark");
    println!("  /plugin disable mindtrace");
    println!("  /journal");

    use_command("/theme dark");
}

pub fn use_command(input: &str) {
    let mut commands = HashMap::new();

    commands.insert("/theme dark", || println!("Theme changed to dark"));
    commands.insert("/plugin disable mindtrace", || println!("mindtrace plugin disabled"));
    commands.insert("/journal", || crate::zen::start_journal().unwrap());
    commands.insert("/copy", || crate::clipboard::copy_node("example node"));
    commands.insert("/workspace switch main", || crate::clipboard::switch_workspace("main"));

    match commands.get(input.trim()) {
        Some(action) => action(),
        None => println!("Unknown command: {}", input),
    }
}
