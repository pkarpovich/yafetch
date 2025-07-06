#[cfg(target_os = "macos")]
use std::process::Command;

pub fn get() -> String {
    #[cfg(target_os = "macos")]
    {
        // Use pmset to get battery information on macOS
        match Command::new("pmset").arg("-g").arg("batt").output() {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                parse_macos_battery(&output_str)
            }
            Err(_) => "No battery".to_string(),
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Check for battery in /sys/class/power_supply/
        parse_linux_battery()
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        "Unsupported".to_string()
    }
}

#[cfg(target_os = "macos")]
fn parse_macos_battery(output: &str) -> String {
    // Parse pmset output
    // Example: "InternalBattery-0 (id=1234567)	100%; charged; 0:00 remaining present: true"
    // or: "InternalBattery-0 (id=1234567)	78%; discharging; 4:15 remaining present: true"
    
    for line in output.lines() {
        if line.contains("InternalBattery") {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let info_parts: Vec<&str> = parts[1].split(';').collect();
                if !info_parts.is_empty() {
                    let percentage = info_parts[0].trim();
                    let status = if info_parts.len() > 1 {
                        let status_str = info_parts[1].trim();
                        match status_str {
                            "charged" => " (charged)",
                            "charging" => " (charging)",
                            "discharging" => "",
                            _ => "",
                        }
                    } else {
                        ""
                    };
                    return format!("{}{}", percentage, status);
                }
            }
        }
    }
    
    "No battery".to_string()
}

#[cfg(target_os = "linux")]
fn parse_linux_battery() -> String {
    use std::fs;
    use std::path::Path;
    
    let battery_path = Path::new("/sys/class/power_supply/BAT0");
    if !battery_path.exists() {
        // Try BAT1
        let battery_path = Path::new("/sys/class/power_supply/BAT1");
        if !battery_path.exists() {
            return "No battery".to_string();
        }
    }
    
    let capacity = match fs::read_to_string(battery_path.join("capacity")) {
        Ok(cap) => cap.trim().to_string(),
        Err(_) => return "No battery".to_string(),
    };
    
    let status = match fs::read_to_string(battery_path.join("status")) {
        Ok(stat) => match stat.trim() {
            "Charging" => " (charging)",
            "Full" => " (charged)",
            "Discharging" => "",
            _ => "",
        },
        Err(_) => "",
    };
    
    format!("{}%{}", capacity, status)
}