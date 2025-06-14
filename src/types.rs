use serde::{Serialize, Deserialize};

/// Global report containing all system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryReport {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disks: Vec<DiskStat>,
    pub network: NetworkStat,
}

/// Stats for an individual disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStat {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
}

/// Stats for aggregated network interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStat {
    pub total_received: u64,
    pub total_transmitted: u64,
}
