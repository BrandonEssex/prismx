#[macro_export]
macro_rules! log_debug {
    ($state:expr, $($arg:tt)*) => {
        if $state.debug_input_mode {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        eprintln!("[INFO] {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!("[WARN] {}", format!($($arg)*));
    };
}
