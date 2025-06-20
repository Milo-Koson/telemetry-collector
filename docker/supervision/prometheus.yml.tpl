global:
  scrape_interval: 5s

scrape_configs:
  - job_name: 'telemetry'
    metrics_path: "${TELEMETRY_METRICS_PATH}"
    static_configs:
      - targets: ['host.docker.internal:${TELEMETRY_PORT}']