use std::collections::HashMap;

pub fn launch_spotlight() {
    println!("[SPOTLIGHT] Command bar active. Try:");
    println!("  /theme dark");
    println!("  /plugin disable mindtrace");
    println!("  /journal");
    println!("  /copy");
    println!("  /workspace switch main");

    let mut commands: HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    commands.insert("/theme dark", Box::new(|| {
        println!("Theme changed to dark");
    }));

    commands.insert("/plugin disable mindtrace", Box::new(|| {
        println!("mindtrace plugin disabled");
    }));

    commands.insert("/journal", Box::new(|| {
        crate::zen::start_journal().unwrap();
    }));

    commands.insert("/copy", Box::new(|| {
        crate::clipboard::copy_node("example node");
    }));

    commands.insert("/workspace switch main", Box::new(|| {
        crate::clipboard::switch_workspace("main");
    }));

    use_command("/theme dark", &commands);
}

pub fn use_command(input: &str, map: &HashMap<&str, Box<dyn Fn()>>) {
    match map.get(input.trim()) {
        Some(action) => action(),
        None => println!("Unknown command: {}", input),
    }
}
