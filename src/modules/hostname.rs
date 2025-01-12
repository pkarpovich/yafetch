use sysinfo::System;

pub fn get() -> String {
    match System::host_name() {
        Some(hostname) => hostname,
        None => "Unknown".to_string(),
    }
}
