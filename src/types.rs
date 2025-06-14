use serde::{Serialize, Deserialize};

/// Rapport global contenant toutes les métriques système
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryReport {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disks: Vec<DiskStat>,
    pub network: NetworkStat,
}

/// Statistiques pour un disque individuel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStat {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
}

/// Statistiques agrégées pour l'ensemble des interfaces réseau
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStat {
    pub total_received: u64,
    pub total_transmitted: u64,
}
