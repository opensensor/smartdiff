# Smart Code Diff - Development Makefile

.PHONY: build test clean check fmt clippy install dev start stop start-backend start-frontend

# Default target
all: check test build

# Build all crates
build:
	cargo build --release

# Run tests
test:
	cargo test --all

# Clean build artifacts
clean:
	cargo clean

# Check code without building
check:
	cargo check --all

# Format code
fmt:
	cargo fmt --all

# Run clippy linter
clippy:
	cargo clippy --all -- -D warnings

# Install binaries
install:
	cargo install --path crates/cli
	cargo install --path crates/web-ui

# Development mode - watch for changes
dev:
	cargo watch -x "check --all" -x "test --all"

# Run the CLI
run-cli:
	cargo run --bin smart-diff

# Run the web server
run-server:
	cargo run --bin smart-diff-server

# Start both backend and frontend (recommended)
start:
	@echo "Starting Smart Code Diff..."
	@./start.sh

# Quick development start
dev-start:
	@echo "Starting in development mode..."
	@./dev.sh

# Stop all services
stop:
	@echo "Stopping all services..."
	@./stop.sh

# Start only backend
start-backend:
	@echo "Starting backend on port 8080..."
	@RUST_LOG=info cargo run --bin smart-diff-server

# Start only frontend
start-frontend:
	@echo "Starting frontend on port 3000..."
	@cd nextjs-frontend && npm run dev

# Install frontend dependencies
install-frontend:
	@echo "Installing frontend dependencies..."
	@cd nextjs-frontend && npm install

# Build frontend
build-frontend:
	@echo "Building frontend..."
	@cd nextjs-frontend && npm run build

# Full setup (first time)
setup: install-frontend build
	@echo "Setup complete! Run 'make start' to start services."

# Generate documentation
docs:
	cargo doc --all --no-deps --open

# Run benchmarks
bench:
	cargo bench --all
