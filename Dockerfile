# Multi-stage Dockerfile for Rusty SaaS
# Optimized for production deployment with security hardening

# Build stage
FROM rust:1.91-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY config ./config

# Build the application with optimizations and strip symbols
RUN touch src/main.rs && \
    cargo build --release && \
    strip target/release/rusty_saas

# Runtime stage - minimal Debian image
FROM debian:bookworm-slim

WORKDIR /app

# Install only essential runtime dependencies and security updates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && apt-get upgrade -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/rusty_saas /app/rusty_saas
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/config /app/config

# Create non-root user with specific UID/GID for security
RUN groupadd -r appuser -g 1000 && \
    useradd -r -u 1000 -g appuser -m -d /home/appuser -s /sbin/nologin appuser && \
    chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Set production environment variables
ENV RUST_LOG=info \
    RUST_BACKTRACE=0

# Health check using the /health endpoint
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["/app/rusty_saas"]
