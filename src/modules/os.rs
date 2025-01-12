use sysinfo::System;

pub fn get() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let name = match System::name() {
        Some(name) if name == "Darwin" => "macOS".to_string(),
        Some(name) => name,
        None => "Unknown".to_string(),
    };

    let version = match System::os_version() {
        Some(v) => v,
        None => "Unknown".to_string(),
    };

    let codname = match version.as_str() {
        "14.0" | "14.1" | "14.2" | "14.3" | "14.4" => "Sonoma".to_string(),
        "15.0" | "15.1" | "15.2" | "15.3" | "15.4" => "Sequoia".to_string(),
        _ => "Unknown".to_string(),
    };

    let kernel = match System::kernel_version() {
        Some(k) => k,
        None => "Unknown".to_string(),
    };

    format!("{name} {codname} (Version {version}, Kernel {kernel})")
}
