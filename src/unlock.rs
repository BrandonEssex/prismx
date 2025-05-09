use std::io::{self, Write};

pub fn prompt_unlock() -> bool {
    print!("Enter unlock passphrase: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        if input.trim() == "letmein" {
            println!("Access granted.");
            return true;
        }
    }

    println!("Access denied.");
    false
}