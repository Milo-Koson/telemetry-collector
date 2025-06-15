use once_cell::sync::Lazy;
use prometheus::{Gauge, IntGauge, register_gauge, register_int_gauge};
use sysinfo::{Networks, System};

// Metrics declaration

pub static CPU_USAGE: Lazy<Gauge> =
    Lazy::new(|| register_gauge!("cpu_usage_percent", "CPU usage in percent").unwrap());
pub static MEMORY_TOTAL: Lazy<IntGauge> =
    Lazy::new(|| register_int_gauge!("memory_total_bytes", "Total memory in bytes").unwrap());
pub static MEMORY_USED: Lazy<IntGauge> =
    Lazy::new(|| register_int_gauge!("memory_used_bytes", "Used memory in bytes").unwrap());

pub static NETWORK_RECEIVED: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "network_received_bytes_total",
        "Total network received bytes"
    )
    .unwrap()
});
pub static NETWORK_TRANSMITTED: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "network_transmitted_bytes_total",
        "Total network transmitted bytes"
    )
    .unwrap()
});

/// Update the metrics with the latest system information
pub fn update_metrics() {
    let mut sys = System::new_all();
    sys.refresh_all();

    CPU_USAGE.set(sys.global_cpu_usage() as f64);

    MEMORY_TOTAL.set(sys.total_memory() as i64 * 1024);
    MEMORY_USED.set(sys.used_memory() as i64 * 1024);

    let networks = Networks::new_with_refreshed_list();
    let mut total_received = 0;
    let mut total_transmitted = 0;

    for data in networks.list().values() {
        total_received += data.received();
        total_transmitted += data.transmitted();
    }

    NETWORK_RECEIVED.set(total_received as i64);
    NETWORK_TRANSMITTED.set(total_transmitted as i64);
}
