use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub metrics_path: String,
}

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_METRICS_PATH: &str = "/metrics";

impl Config {
    /// Loads the configuration from environment variables.
    pub fn load() -> Self {
        let port = env::var("TELEMETRY_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_PORT);

        let metrics_path =
            env::var("TELEMETRY_METRICS_PATH").unwrap_or_else(|_| DEFAULT_METRICS_PATH.to_string());

        Config { port, metrics_path }
    }
}
