use systemstat::{Platform, System};

pub fn get() -> String {
    let system = System::new();
    match system.uptime() {
        Ok(uptime) => {
            let uptime_seconds = uptime.as_secs();
            let days = uptime_seconds / 86_400;
            let hours = (uptime_seconds % 86_400) / 3600;
            let minutes = (uptime_seconds % 3600) / 60;

            match (days, hours) {
                (0, 0) => format!("{minutes}m"),
                (0, _) => format!("{hours}h {minutes}m"),
                (_, _) => format!("{days}d {hours}h {minutes}m"),
            }
        }
        Err(e) => format!("Failed to fetch uptime: {}", e),
    }
}
