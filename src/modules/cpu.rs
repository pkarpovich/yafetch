use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub fn get() -> String {
    let system =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

    if let Some(cpu) = system.cpus().first() {
        let brand = cpu.brand();
        let vendor = cpu.vendor_id();
        format!("{vendor} {brand}")
    } else {
        "Failed to fetch CPU information".to_string()
    }
}
