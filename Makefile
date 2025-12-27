.PHONY: help infra-up infra-down infra-up-all infra-down-all dbmate-up dbmate-down dbmate-status

COMPOSE := docker compose -f infra/local/base.yaml

help:
	@echo "Targets:"
	@echo "  infra-local-pg-up           Start local postgres (only)"
	@echo "  infra-local-pg-down         Stop local postgres (only)"
	@echo "  infra-local-up       Start full local stack"
	@echo "  infra-local-down     Stop full local stack"
	@echo "  dbmate-up          Apply all pending migrations"
	@echo "  dbmate-down N=1    Roll back the last N migrations"
	@echo "  dbmate-status      Show migration status"

infra-local-pg-up:
	$(COMPOSE) up -d postgres

infra-local-pg-down:
	$(COMPOSE) stop postgres

infra-local-up:
	$(COMPOSE) up -d

infra-local-down:
	$(COMPOSE) down

# Runs dbmate in a one-shot container on the same compose network.
# Requires the `dbmate` service defined in infra/local/base.yaml.

dbmate-up: infra-up
	$(COMPOSE) run --rm dbmate up

dbmate-down: infra-up
	$(COMPOSE) run --rm dbmate down $(N)

dbmate-status: infra-up
	$(COMPOSE) run --rm dbmate status
