use sysinfo::System;

const BYTES_TO_GIB: f64 = 1.0 / 1024.0 / 1024.0 / 1024.0;

pub fn get_used() -> String {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let used_memory_gb = sys.used_memory() as f64 * BYTES_TO_GIB;
    format!("{:.1} GB", used_memory_gb)
}

pub fn get_total() -> String {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_memory_gib = sys.total_memory() as f64 * BYTES_TO_GIB;
    format!("{:.1} GiB", total_memory_gib)
}
