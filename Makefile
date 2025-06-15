TELEMETRY_COMPOSE=docker/docker-compose.yml

.PHONY: up up-build down logs clean

up:
	docker compose -f $(TELEMETRY_COMPOSE) up -d

up-build:
	docker compose -f $(TELEMETRY_COMPOSE) up -d --build

down:
	docker compose -f $(TELEMETRY_COMPOSE) down

logs:
	docker compose -f $(TELEMETRY_COMPOSE) logs -f

clean: down
	docker volume prune -f
	docker network prune -f