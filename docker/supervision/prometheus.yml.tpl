global:
  scrape_interval: 5s

scrape_configs:
  - job_name: 'system_metrics'
    metrics_path: "${SYSTEM_METRICS_PATH}"
    static_configs:
      - targets: ['host.docker.internal:${TELEMETRY_PORT}']
  - job_name: 'status_metrics'
    metrics_path: "${STATUS_METRICS_PATH}"
    static_configs:
      - targets: ['host.docker.internal:${TELEMETRY_PORT}']