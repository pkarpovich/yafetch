use fs2;

const BYTES_TO_GIB: f64 = 1.0 / 1024.0 / 1024.0 / 1024.0;
const BYTES_TO_TB: f64 = 1.0 / 1024.0 / 1024.0 / 1024.0 / 1024.0;

pub fn get_total(path: String) -> String {
    match fs2::total_space(&path) {
        Ok(total) => format!("{:.1} TB", total as f64 * BYTES_TO_TB),
        Err(e) => format!("Failed to get total space: {}", e),
    }
}

pub fn get_free(path: String) -> String {
    match fs2::available_space(&path) {
        Ok(free) => format!("{:.1} GB", free as f64 * BYTES_TO_GIB),
        Err(e) => format!("Failed to get free space: {}", e),
    }
}
