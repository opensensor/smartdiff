# Smart Code Diff - Development Makefile

.PHONY: build test clean check fmt clippy install dev

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

# Generate documentation
docs:
	cargo doc --all --no-deps --open

# Run benchmarks
bench:
	cargo bench --all
