use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub system_metrics_path: String,
    pub status_metrics_path: String,
}

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_SYSTEM_METRICS_PATH: &str = "/metrics/system";
const DEFAULT_STATUS_METRICS_PATH: &str = "/metrics/status";

impl Config {
    /// Loads the configuration from environment variables.
    pub fn load() -> Self {
        let port = env::var("TELEMETRY_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_PORT);

        let system_metrics_path = env::var("SYSTEM_METRICS_PATH")
            .unwrap_or_else(|_| DEFAULT_SYSTEM_METRICS_PATH.to_string());
        let status_metrics_path = env::var("STATUS_METRICS_PATH")
            .unwrap_or_else(|_| DEFAULT_STATUS_METRICS_PATH.to_string());

        Config {
            port,
            system_metrics_path,
            status_metrics_path,
        }
    }
}
