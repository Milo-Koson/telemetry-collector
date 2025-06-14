use sysinfo::{Disks, Networks, System};
use crate::types::{TelemetryReport, DiskStat, NetworkStat};

/// Gathering CPU usage in percentage
pub fn collect_cpu_usage(sys: &System) -> f32 {
    sys.global_cpu_usage()
}

/// Gathering memory statistics
pub fn collect_memory_usage(sys: &System) -> (u64, u64) {
    (sys.total_memory(), sys.used_memory())
}

/// Gathering disk statistics
pub fn collect_disk_usage() -> Vec<DiskStat> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_stats = Vec::new();
    for disk in disks.list() {
        let disk_stat = DiskStat {
            name: disk.name().to_string_lossy().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        };
        disk_stats.push(disk_stat);
    }
    disk_stats
}
/// Gathering network statistics
pub fn collect_network_usage() -> NetworkStat {
    let networks = Networks::new_with_refreshed_list();
    let mut total_received = 0;
    let mut total_transmitted = 0;

    for data in networks.list().values() {
        total_received += data.received();
        total_transmitted += data.transmitted();
    }
    NetworkStat {
        total_received,
        total_transmitted,  
    }
}
/// Gathering all collected metrics into a single report
pub fn gather_all_metrics() -> TelemetryReport {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = collect_cpu_usage(&sys);
    let (memory_total, memory_used) = collect_memory_usage(&sys);
    let disks = collect_disk_usage();
    let network = collect_network_usage();

    TelemetryReport {
        cpu_usage,
        memory_total,
        memory_used,
        disks,
        network,
    }
}