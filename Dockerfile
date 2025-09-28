# Multi-stage Docker build for Smart Code Diff

# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build dependencies (this is cached if dependencies don't change)
RUN cargo build --release --bin smart-diff-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false smartdiff

# Copy binary from builder stage
COPY --from=builder /app/target/release/smart-diff-server /usr/local/bin/smart-diff-server

# Set ownership and permissions
RUN chown smartdiff:smartdiff /usr/local/bin/smart-diff-server
RUN chmod +x /usr/local/bin/smart-diff-server

# Switch to app user
USER smartdiff

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/api/health || exit 1

# Run the server
CMD ["smart-diff-server"]
