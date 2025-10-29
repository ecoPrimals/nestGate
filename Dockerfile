# Multi-stage build for optimal container size and security
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 nestgate

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY code/crates ./code/crates

# Build dependencies (cached layer)
RUN cargo build --release --workspace

# Copy source code
COPY . .

# Build application
RUN cargo build --release --workspace

# Runtime stage
FROM debian:bookworm-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 nestgate

# Create directories
RUN mkdir -p /app/data /app/logs /app/config \
    && chown -R nestgate:nestgate /app

# Copy binary from builder
COPY --from=builder /app/target/release/nestgate-api /usr/local/bin/nestgate-api
COPY --from=builder /app/target/release/nestgate-installer /usr/local/bin/nestgate-installer

# Copy configuration
COPY --chown=nestgate:nestgate config/ /app/config/

# Switch to non-root user
USER nestgate

# Set working directory
WORKDIR /app

# Expose ports
EXPOSE 8080 8081 8082

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Environment variables
ENV RUST_LOG=info
ENV NESTGATE_CONFIG_PATH=/app/config
ENV NESTGATE_DATA_PATH=/app/data
ENV NESTGATE_LOG_PATH=/app/logs

# Default command
CMD ["nestgate-api"] 