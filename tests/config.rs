use std::env;
use telemetry_collector::config::Config;

#[test]
fn test_config_from_env_vars() {
    unsafe {
        env::set_var("TELEMETRY_PORT", "9000");
        env::set_var("TELEMETRY_METRICS_PATH", "/test-metrics");
    }

    let config = Config::load();
    assert_eq!(config.port, 9000);
    assert_eq!(config.metrics_path, "/test-metrics");

    unsafe {
        env::remove_var("TELEMETRY_PORT");
        env::remove_var("TELEMETRY_METRICS_PATH");
    }
}

#[test]
fn test_config_defaults_when_env_missing() {
    unsafe {
        env::remove_var("TELEMETRY_PORT");
        env::remove_var("TELEMETRY_METRICS_PATH");
    }

    let config = Config::load();

    assert_eq!(config.port, 8080); // Default
    assert_eq!(config.metrics_path, "/metrics"); // Default
}