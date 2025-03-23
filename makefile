run-server:
	cargo run --manifest-path apps/server/Cargo.toml

run-client:
	cd apps/client && pnpm dev

run-db:
	docker-compose up -d

migrate:
	cargo sqlx migrate run
