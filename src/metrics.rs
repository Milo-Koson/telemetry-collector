use sysinfo::{Disks, Networks, System};
use crate::types::{TelemetryReport, DiskStat, NetworkStat};
use std::io::{Error, ErrorKind};

/// Gathering CPU usage in percentage
pub fn collect_cpu_usage(sys: &System) -> Result<f32, Error> {
    Ok(sys.global_cpu_usage())
}

/// Gathering memory statistics
pub fn collect_memory_usage(sys: &System) -> Result<(u64, u64), Error> {
    Ok((sys.total_memory(), sys.used_memory()))
}

/// Gathering disk statistics
pub fn collect_disk_usage() -> Result<Vec<DiskStat>, Error> {
    let disks = Disks::new_with_refreshed_list();
    if disks.list().is_empty() {
        return Err(Error::new(ErrorKind::NotFound, "No disks found"));
    }
    
    let mut disk_stats = Vec::new();
    for disk in disks.list() {
        let disk_stat = DiskStat {
            name: disk.name().to_string_lossy().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        };
        disk_stats.push(disk_stat);
    }
    Ok(disk_stats)
}

/// Gathering network statistics
pub fn collect_network_usage() -> Result<NetworkStat, Error> {
    let networks = Networks::new_with_refreshed_list();
    if networks.list().is_empty() {
        return Err(Error::new(ErrorKind::NotFound, "No network interfaces found"));
    }

    let mut total_received = 0;
    let mut total_transmitted = 0;

    for data in networks.list().values() {
        total_received += data.received();
        total_transmitted += data.transmitted();
    }
    Ok(NetworkStat {
        total_received,
        total_transmitted,  
    })
}

/// Gathering all collected metrics into a single report
pub fn gather_all_metrics() -> Result<TelemetryReport, Error> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = collect_cpu_usage(&sys)?;
    let (memory_total, memory_used) = collect_memory_usage(&sys)?;
    let disks = collect_disk_usage()?;
    let network = collect_network_usage()?;

    Ok(TelemetryReport {
        cpu_usage,
        memory_total,
        memory_used,
        disks,
        network,
    })
}