use chrono::Local;

pub fn get() -> String {
    let now = Local::now();
    now.format("%H:%M:%S %Y-%m-%d").to_string()
}
