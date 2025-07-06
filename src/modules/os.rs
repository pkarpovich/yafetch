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

    let codname = if let Some(major_version) = version.split('.').next() {
        match major_version {
            "14" => "Sonoma".to_string(),
            "15" => "Sequoia".to_string(),
            "26" => "Tahoe".to_string(),
            _ => "Unknown".to_string(),
        }
    } else {
        "Unknown".to_string()
    };

    let kernel = match System::kernel_version() {
        Some(k) => k,
        None => "Unknown".to_string(),
    };

    format!("{name} {codname} (Version {version}, Kernel {kernel})")
}
