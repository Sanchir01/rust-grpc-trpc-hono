PHONY:

SILENT:

include .env

DIR_CH = migrations/clickhouse
DIR_PG = ./migrations/postgres

CLICKHOUSE_DRIVER := clickhouse
CLICKHOUSE_USER := default
CLICKHOUSE_HOST := localhost
CLICKHOUSE_PORT := 8123
CLICKHOUSE_DB := default

CLICKHOUSE_DSN := "tcp://$(CLICKHOUSE_HOST):$(CLICKHOUSE_PORT)?database=$(CLICKHOUSE_DB)&username=$(CLICKHOUSE_USER)"

POSTGRES_USER := postgres
POSTGRES_PASSWORD := $(POSTGRES_PASSWORD)
POSTGRES_HOST := localhost
POSTGRES_PORT := 5432
POSTGRES_DB := postgres

POSTGRES_DSN := "postgresql://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)"

compose:
	docker-compose up -d
dockerfile:
	docker build -t trpc-rust-grpc .

ingester:
	cargo watch -x 'run -p ingester'

generator:
	cargo watch -x 'run -p generator'
run-prod:
	cargo run --release -p ingester &  cargo run --release -p generator

workspace:
	cargo watch -x 'run -p generator' & cargo watch -x 'run -p ingester'
	
migrations-up-pg:
	goose -dir $(DIR_PG) postgres  $(POSTGRES_DSN) up

migrations-down-pg:
	goose -dir $(DIR_PG) postgres  $(POSTGRES_DSN) down


migrations-status-pg:
	goose -dir $(DIR_PG) postgres  $(POSTGRES_DSN) status

migrations-new-pg:
	goose -dir $(DIR_PG) create $(MIGRATION_NAME) sql

migrations-new-ch:
	goose -dir $(DIR_CH) create $(MIGRATION_NAME) sql


