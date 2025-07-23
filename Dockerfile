# ==============================================================================
# NestGate Production Dockerfile
# Multi-stage build for optimal security and performance
# ==============================================================================

# Build Stage - Using official Rust image
FROM rust:1.75-slim as builder

LABEL maintainer="NestGate Team"
LABEL description="NestGate Storage System - Production Build"

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libzfslinux-dev \
    build-essential \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/nestgate

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY code/crates/nestgate-core/Cargo.toml ./code/crates/nestgate-core/
COPY code/crates/nestgate-api/Cargo.toml ./code/crates/nestgate-api/
COPY code/crates/nestgate-bin/Cargo.toml ./code/crates/nestgate-bin/
COPY code/crates/nestgate-zfs/Cargo.toml ./code/crates/nestgate-zfs/
COPY code/crates/nestgate-automation/Cargo.toml ./code/crates/nestgate-automation/
COPY code/crates/nestgate-network/Cargo.toml ./code/crates/nestgate-network/
COPY code/crates/nestgate-mcp/Cargo.toml ./code/crates/nestgate-mcp/

# Build dependencies (cached layer)
RUN cargo fetch

# Copy source code
COPY code/ ./code/
COPY benches/ ./benches/
COPY tests/ ./tests/

# Build for production with optimizations
ENV RUSTFLAGS="-C target-cpu=native -C opt-level=3"
RUN cargo build --release --bin nestgate

# Runtime Stage - Minimal Debian image
FROM debian:bookworm-slim

LABEL maintainer="NestGate Team"
LABEL description="NestGate Storage System - Production Runtime"
LABEL version="2.0.0"

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    zfsutils-linux \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user for security
RUN groupadd -r nestgate && useradd -r -g nestgate nestgate

# Create necessary directories
RUN mkdir -p /opt/nestgate/{bin,config,data,logs} \
    && chown -R nestgate:nestgate /opt/nestgate

# Copy binary from builder stage
COPY --from=builder /usr/src/nestgate/target/release/nestgate /opt/nestgate/bin/

# Copy production configuration template
COPY --chown=nestgate:nestgate docker/production.toml /opt/nestgate/config/default.toml

# Set working directory and user
WORKDIR /opt/nestgate
USER nestgate

# Environment variables for production
ENV NESTGATE_ENVIRONMENT=production
ENV NESTGATE_CONFIG_DIR=/opt/nestgate/config
ENV NESTGATE_DATA_DIR=/opt/nestgate/data
ENV NESTGATE_LOG_DIR=/opt/nestgate/logs
ENV NESTGATE_LOG_LEVEL=info
ENV NESTGATE_API_PORT=8000
ENV NESTGATE_WEBSOCKET_PORT=8080
ENV RUST_LOG=nestgate=info
ENV RUST_BACKTRACE=1

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${NESTGATE_API_PORT}/health || exit 1

# Expose ports
EXPOSE 8000 8080 9090

# Volume mounts for persistence
VOLUME ["/opt/nestgate/data", "/opt/nestgate/logs", "/opt/nestgate/config"]

# Entrypoint
ENTRYPOINT ["/opt/nestgate/bin/nestgate"]
CMD ["--config", "/opt/nestgate/config/default.toml"] 