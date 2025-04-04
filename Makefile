PHONY:

SILENT:


compose:
	docker-compose up


ingester:
	cargo watch -x 'run -p ingester'

generator:
	cargo watch -x 'run -p generator'

rust:
	cargo watch -x 'run -p generator' & cargo watch -x 'run -p ingester'
	