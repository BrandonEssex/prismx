use std::fs;
use walkdir::WalkDir;

fn file_contains(path: &str, pattern: &str) -> bool {
    fs::read_to_string(path).map(|c| c.contains(pattern)).unwrap_or(false)
}

fn search_src(pattern: &str) -> bool {
    for entry in WalkDir::new("src") {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Ok(txt) = fs::read_to_string(e.path()) {
                    if txt.contains(pattern) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn search_src_excluding(pattern: &str, exclude: &str) -> bool {
    for entry in WalkDir::new("src") {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if e.path() == std::path::Path::new(exclude) {
                    continue;
                }
                if let Ok(txt) = fs::read_to_string(e.path()) {
                    if txt.contains(pattern) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn main() {
    let mut ok = true;

    if file_contains("src/main.rs", "init_logger()") || file_contains("src/bootstrap.rs", "init_logger()") {
        println!("✅ init_logger wired");
    } else {
        println!("❌ init_logger() call missing");
        ok = false;
    }

    if file_contains("src/state/init.rs", "load_plugins") {
        println!("✅ load_plugins wired");
    } else {
        println!("❌ load_plugins call missing");
        ok = false;
    }

    if file_contains("src/zen/editor.rs", "toggle_zen_view") || file_contains("src/zen/editor.rs", "/scroll") {
        println!("✅ zen editor toggle present");
    } else {
        println!("❌ toggle_zen_view or /scroll missing");
        ok = false;
    }

    if search_src("ZenViewMode") && search_src("ZenLayoutMode") {
        println!("✅ Zen modes detected");
    } else {
        println!("❌ Zen mode usage missing");
        ok = false;
    }

    if search_src_excluding("entry.entry", "src/bin/audit.rs") {
        println!("❌ old entry.entry reference found");
        ok = false;
    } else {
        println!("✅ no old entry.entry references");
    }

    if ok {
        println!("✅ Audit complete");
    } else {
        println!("❌ Audit completed with issues");
        std::process::exit(1);
    }
}
