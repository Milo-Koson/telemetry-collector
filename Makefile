TELEMETRY_COMPOSE := docker/docker-compose.yml
PROMETHEUS_TEMPLATE := docker/supervision/prometheus.yml.tpl
PROMETHEUS_CONFIG := docker/supervision/prometheus.yml

ENV_FILE := .env
include $(ENV_FILE)
export

.PHONY: up down logs clean generate_prometheus run

generate_prometheus:
	envsubst < $(PROMETHEUS_TEMPLATE) > $(PROMETHEUS_CONFIG)

run:
	cargo build --release --bin telemetry-collector
	cargo run --release --bin telemetry-collector

up: generate_prometheus
	docker compose -f $(TELEMETRY_COMPOSE) up -d
	$(MAKE) run

down:
	docker compose -f $(TELEMETRY_COMPOSE) down

logs:
	docker compose -f $(TELEMETRY_COMPOSE) logs -f

clean: down
		@docker volume rm -f $(shell docker volume ls -q --filter label=com.docker.compose.project=$(shell basename $(dir $(TELEMETRY_COMPOSE))))
		@echo "Everything has been cleaned up."