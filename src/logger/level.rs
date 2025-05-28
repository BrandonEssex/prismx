pub fn log_filter() -> String {
    let level = std::env::var("PRISMX_LOG").unwrap_or_else(|_| "info".to_string());
    match level.to_lowercase().as_str() {
        "debug" => "debug".into(),
        "off" => "off".into(),
        _ => "info".into(),
    }
}
