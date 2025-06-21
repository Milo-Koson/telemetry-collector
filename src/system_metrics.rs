use once_cell::sync::Lazy;
use prometheus::{Gauge, Registry, register_gauge_with_registry};
use std::process::Command;
use sysinfo::System;

// New registry for system metrics
pub static SYSTEM_REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

const KBYTES_IN_GB: f64 = 1024.0 * 1024.0 * 1024.0;
const BYTES_IN_MB: f64 = 1024.0 * 1024.0;

// Metrics declaration
pub static CPU_USAGE: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "cpu_usage_percent",
        "CPU usage in percent",
        &*SYSTEM_REGISTRY
    )
    .unwrap()
});

pub static MEMORY_TOTAL: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "memory_total_gigabytes",
        "Total memory in gigabytes",
        &*SYSTEM_REGISTRY
    )
    .unwrap()
});

pub static MEMORY_USED: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "memory_used_gigabytes",
        "Used memory in gigabytes",
        &*SYSTEM_REGISTRY
    )
    .unwrap()
});

pub static NETWORK_RECEIVED: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "network_received_megabytes_total",
        "Total network received in megabytes",
        &*SYSTEM_REGISTRY
    )
    .unwrap()
});

pub static NETWORK_TRANSMITTED: Lazy<Gauge> = Lazy::new(|| {
    register_gauge_with_registry!(
        "network_transmitted_megabytes_total",
        "Total network transmitted in megabytes",
        &*SYSTEM_REGISTRY
    )
    .unwrap()
});

/// Update the metrics with the latest system information
pub fn update_system_metrics() {
    let mut sys = System::new_all();
    sys.refresh_all();

    CPU_USAGE.set(sys.global_cpu_usage() as f64);

    let total_gb = (sys.total_memory() as f64) / KBYTES_IN_GB;
    let used_gb = (sys.used_memory() as f64) / KBYTES_IN_GB;

    MEMORY_TOTAL.set((total_gb * 100.0).round() / 100.0);
    MEMORY_USED.set((used_gb * 100.0).round() / 100.0);

    let output = Command::new("netstat")
        .arg("-ib")
        .output()
        .expect("Failed to run netstat");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut total_received: u64 = 0;
    let mut total_transmitted: u64 = 0;

    for line in stdout.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 {
            continue;
        }
        if let (Ok(rx), Ok(tx)) = (fields[6].parse::<u64>(), fields[9].parse::<u64>()) {
            total_received += rx;
            total_transmitted += tx;
        }
    }

    let received_mb = (total_received as f64) / BYTES_IN_MB;
    let transmitted_mb = (total_transmitted as f64) / BYTES_IN_MB;

    NETWORK_RECEIVED.set((received_mb * 100.0).round() / 100.0);
    NETWORK_TRANSMITTED.set((transmitted_mb * 100.0).round() / 100.0);
}
