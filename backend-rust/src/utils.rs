use std::env;

pub fn is_local_runtime() -> bool {
    let app_env = env::var("APP_ENV")
        .or_else(|_| env::var("RUST_ENV"))
        .unwrap_or_default()
        .to_lowercase();

    matches!(app_env.as_str(), "local" | "development" | "dev")
        || (cfg!(debug_assertions) && app_env != "production")
}

pub fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => {
                encoded.push('+');
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}
