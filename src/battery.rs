use crate::{extra, PATH_TO_BATTERY_PERCENTAGE, PATH_TO_BATTERY_STATUS};
#[allow(unused_imports)]
use std::{fs, process::Command};

/// Read battery percentage from `/sys/class/power_supply/BAT0/capacity`
#[cfg(target_os = "linux")]
pub fn percentage() -> String {
    let percentage = fs::read_to_string(PATH_TO_BATTERY_PERCENTAGE);
    let ret = match percentage {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

/// Read battery status from `/sys/class/power_supply/BAT0/status`
#[cfg(target_os = "linux")]
pub fn status() -> String {
    let status = fs::read_to_string(PATH_TO_BATTERY_STATUS);
    let ret = match status {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

/// Read battery percentage through `sysctl -nb hw.acpi.battery.life`
#[cfg(target_os = "netbsd")]
pub fn percentage() -> String {
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "hw.acpi.battery.life"])
        .output()
        .expect("ERROR: failed to start \"sysctl\" process");

    let percentage = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(percentage)
}

/// Read battery status through `sysctl -nb hw.acpi.battery.state`
#[cfg(target_os = "netbsd")]
pub fn status() -> String {
    let output = Command::new("sysctl")
        .args(&["-n", "-b", "hw.acpi.battery.state"])
        .output()
        .expect("ERROR: failed to start \"sysctl\" process");

    let percentage = String::from_utf8(output.stdout)
        .expect("ERROR: \"sysctl\" process stdout was not valid UTF-8");
    String::from(percentage)
}
