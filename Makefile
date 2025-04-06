PHONY:

SILENT:


DIR_CLICKHOUSE = migrations/clickhouse
DIR_PG = ./migrations/postgres

CLICKHOUSE_DRIVER := clickhouse
CLICKHOUSE_USER := default
CLICKHOUSE_HOST := localhost
CLICKHOUSE_PORT := 8123
CLICKHOUSE_DB := default

CLICKHOUSE_DSN := "tcp://$(CLICKHOUSE_HOST):$(CLICKHOUSE_PORT)?database=$(CLICKHOUSE_DB)&username=$(CLICKHOUSE_USER)"

compose:
	docker-compose up


ingester:
	cargo watch -x 'run -p ingester'

generator:
	cargo watch -x 'run -p generator'

rust:
	cargo watch -x 'run -p generator' & cargo watch -x 'run -p ingester'
	
migrations-up-pg:
	goose -dir migrations postgres "host=localhost user=postgres password=postgres port=5435 dbname=test sslmode=disable"  up

migrations-down-pg:
	goose -dir migrations postgres  "host=localhost user=postgres password=postgres port=5435 dbname=test sslmode=disable"  down


migrations-status-pg:
	goose -dir migrations postgres  "host=localhost user=postgres password=postgres port=5435 dbname=test sslmode=disable" status

migrations-status-clickhouse:
	goose -dir $(DIR_CLICKHOUSE) clickhouse  $(CLICKHOUSE_DSN) status

migrations-new-clickhouse:
	goose -dir migrations/clickhouse clickhouse "tcp://localhost:9000?db=default&username=default" up

migrations-new-pg:
	goose -dir $(DIR_PG) create $(MIGRATION_NAME) sql


