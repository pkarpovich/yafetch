use std::env;

pub fn get() -> String {
    env::var("USER").unwrap_or_else(|_| "Unknown".to_string())
}
