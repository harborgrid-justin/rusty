.PHONY: help build run test clean fmt lint docker-build docker-up docker-down migrate

# Default target
help:
	@echo "Rusty SaaS - Available targets:"
	@echo "  build         - Build the project"
	@echo "  run           - Run the application"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  fmt           - Format code"
	@echo "  lint          - Run linter"
	@echo "  docker-build  - Build Docker image"
	@echo "  docker-up     - Start Docker Compose services"
	@echo "  docker-down   - Stop Docker Compose services"
	@echo "  migrate       - Run database migrations"

# Build the project
build:
	cargo build --release

# Run the application
run:
	cargo run

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Build Docker image
docker-build:
	docker build -t rusty-saas:latest .

# Start Docker Compose services
docker-up:
	docker-compose up -d

# Stop Docker Compose services
docker-down:
	docker-compose down

# Run database migrations (requires sqlx-cli)
migrate:
	sqlx migrate run

# Development server with auto-reload (requires cargo-watch)
dev:
	cargo watch -x run

# Check code without building
check:
	cargo check

# Generate documentation
docs:
	cargo doc --no-deps --open

# Security audit
audit:
	cargo audit
