watch:
	cargo watch -- cargo run

entity:
	sea-orm-cli generate entity --with-serde both -o src/entity

migrate:
	sqlx mig run
