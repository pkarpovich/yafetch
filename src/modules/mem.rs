use sysinfo::{MemoryRefreshKind, RefreshKind, System};

const BYTES_TO_GIB: f64 = 1.0 / 1024.0 / 1024.0 / 1024.0;

fn memory() -> System {
    System::new_with_specifics(RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()))
}

pub fn get_used() -> String {
    let used_memory_gb = memory().used_memory() as f64 * BYTES_TO_GIB;
    format!("{:.1} GB", used_memory_gb)
}

pub fn get_total() -> String {
    let total_memory_gib = memory().total_memory() as f64 * BYTES_TO_GIB;
    format!("{:.1} GiB", total_memory_gib)
}
