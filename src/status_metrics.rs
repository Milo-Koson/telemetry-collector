use once_cell::sync::Lazy;
use prometheus::{Gauge, Registry, register_gauge_with_registry};
use regex::Regex;
use std::process::Command;

// New registry for status metrics
pub static STATUS_REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

// Declaration of status metrics
pub static WIFI_ENABLED: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "status_wifi_enabled",
        "Wi-Fi status: 1 = enabled, 0 = disabled",
        &*STATUS_REGISTRY
    )
    .unwrap()
});

pub static BLUETOOTH_ENABLED: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "status_bluetooth_enabled",
        "Bluetooth status: 1 = enabled, 0 = disabled",
        &*STATUS_REGISTRY
    )
    .unwrap()
});

pub static BATTERY_LEVEL: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "status_battery_percentage",
        "Battery level as a percentage",
        &*STATUS_REGISTRY
    )
    .unwrap()
});

static BATTERY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)%").unwrap());

// Update status metrics
pub fn update_status_metrics() {
    // Checks if Wi-Fi is enabled
    if let Ok(output) = Command::new(
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
    )
    .arg("-I")
    .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let wifi_active = stdout.contains("state: running");
        WIFI_ENABLED.set(wifi_active as i32 as f64);
    }

    // Checks if Bluetooth is enabled
    if let Ok(output) = Command::new("system_profiler")
        .arg("SPBluetoothDataType")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let bt_active = stdout.contains("State: On");
        BLUETOOTH_ENABLED.set(bt_active as i32 as f64);
    }

    // Checks battery level
    if let Ok(output) = Command::new("pmset").arg("-g").arg("batt").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("InternalBattery") {
                if let Some(caps) = BATTERY_RE.captures(line) {
                    if let Some(m) = caps.get(1) {
                        if let Ok(value) = m.as_str().parse::<f64>() {
                            BATTERY_LEVEL.set(value);
                        }
                    }
                }
            }
        }
    }
}
