#!/bin/bash

# ==============================================================================
# NestGate Production Deployment Script
# Automated setup and deployment with monitoring
# ==============================================================================

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEPLOY_ENV="${DEPLOY_ENV:-production}"
NESTGATE_VERSION="${NESTGATE_VERSION:-latest}"
LOG_LEVEL="${LOG_LEVEL:-info}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Header
echo "=============================================================================="
echo "🚀 NestGate Production Deployment"
echo "Environment: ${DEPLOY_ENV}"
echo "Version: ${NESTGATE_VERSION}"
echo "=============================================================================="

# Preflight checks
log_info "Running preflight checks..."

# Check Docker
if ! command -v docker &> /dev/null; then
    log_error "Docker is not installed or not in PATH"
    exit 1
fi

# Check Docker Compose
if ! command -v docker-compose &> /dev/null; then
    log_error "Docker Compose is not installed or not in PATH"
    exit 1
fi

# Check ZFS (if available)
if command -v zfs &> /dev/null; then
    log_success "ZFS utilities detected"
    ZFS_AVAILABLE=true
else
    log_warn "ZFS utilities not found - ZFS features will be limited"
    ZFS_AVAILABLE=false
fi

# Check required directories
log_info "Creating required directories..."
sudo mkdir -p /opt/nestgate/{data,logs,config}
sudo mkdir -p /opt/nestgate/data/{hot,warm,cold}
sudo chown -R $USER:$USER /opt/nestgate

# Set up environment variables
log_info "Setting up environment..."

# Create .env file if it doesn't exist
if [ ! -f "${SCRIPT_DIR}/.env" ]; then
    log_info "Creating default environment configuration..."
    cat > "${SCRIPT_DIR}/.env" << EOF
# NestGate Production Environment
NESTGATE_ENVIRONMENT=production
NESTGATE_LOG_LEVEL=${LOG_LEVEL}
NESTGATE_VERSION=${NESTGATE_VERSION}

# Networking
NESTGATE_API_PORT=8000
NESTGATE_WEBSOCKET_PORT=8080
NESTGATE_METRICS_PORT=9090

# Storage Configuration
NESTGATE_DEFAULT_CACHE_SIZE=2147483648
NESTGATE_MAX_FILE_SIZE=107374182400
NESTGATE_CONNECTION_TIMEOUT_MS=30000
NESTGATE_REQUEST_TIMEOUT_MS=60000

# Security
GRAFANA_ADMIN_PASSWORD=nestgate_admin_2024

# Performance
RUST_LOG=nestgate=info
RUST_BACKTRACE=1
EOF
    log_success "Environment configuration created at ${SCRIPT_DIR}/.env"
else
    log_info "Using existing environment configuration"
fi

# Build the application
log_info "Building NestGate application..."
if docker-compose build --no-cache nestgate; then
    log_success "Application built successfully"
else
    log_error "Failed to build application"
    exit 1
fi

# Start the monitoring stack first
log_info "Starting monitoring infrastructure..."
if docker-compose up -d prometheus grafana loki; then
    log_success "Monitoring stack started"
else
    log_error "Failed to start monitoring stack"
    exit 1
fi

# Wait for monitoring to be ready
log_info "Waiting for monitoring services to be ready..."
sleep 10

# Start NestGate
log_info "Starting NestGate main service..."
if docker-compose up -d nestgate; then
    log_success "NestGate service started"
else
    log_error "Failed to start NestGate service"
    exit 1
fi

# Wait for application to be ready
log_info "Waiting for NestGate to be ready..."
timeout=60
counter=0
while ! curl -f -s http://localhost:8000/health > /dev/null 2>&1; do
    if [ $counter -ge $timeout ]; then
        log_error "NestGate failed to start within ${timeout} seconds"
        log_info "Checking logs..."
        docker-compose logs nestgate
        exit 1
    fi
    counter=$((counter + 1))
    sleep 1
done

log_success "NestGate is ready and responding to health checks"

# Display service information
echo ""
echo "=============================================================================="
echo "🎉 NestGate Deployment Complete!"
echo "=============================================================================="
echo ""
log_info "Service Endpoints:"
echo "  • NestGate API:      http://localhost:8000"
echo "  • WebSocket:         http://localhost:8080"
echo "  • Health Check:      http://localhost:8000/health"
echo "  • Metrics:           http://localhost:9090/metrics"
echo ""
log_info "Monitoring Dashboard:"
echo "  • Grafana:           http://localhost:3000"
echo "  • Prometheus:        http://localhost:9091"
echo "  • Loki (Logs):       http://localhost:3100"
echo ""
log_info "Default Credentials:"
echo "  • Grafana:           admin / nestgate_admin_2024"
echo ""

# Display container status
log_info "Container Status:"
docker-compose ps

# Display resource usage
log_info "Resource Usage:"
docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"

echo ""
log_success "Deployment completed successfully!"
log_info "Monitor logs with: docker-compose logs -f"
log_info "Stop services with: docker-compose down"

echo "==============================================================================" 