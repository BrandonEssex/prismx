use std::io::{self, Write};

pub fn prompt_unlock() -> bool {
    print!("Enter unlock passphrase: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    if let Ok(_) = io::stdin().read_line(&mut buf) {
        if buf.trim() == "letmein" {
            println!("Unlock successful.");
            return true;
        }
    }

    println!("Access denied.");
    false
}