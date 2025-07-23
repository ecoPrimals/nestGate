#!/bin/bash

# ==============================================================================
# NestGate Health Check Script
# Comprehensive production health monitoring
# ==============================================================================

set -euo pipefail

# Configuration
NESTGATE_HOST="${NESTGATE_HOST:-localhost}"
NESTGATE_PORT="${NESTGATE_PORT:-8000}"
TIMEOUT="${TIMEOUT:-10}"
VERBOSE="${VERBOSE:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Logging
log_info() {
    if [ "$VERBOSE" = "true" ]; then
        echo -e "[INFO] $1"
    fi
}

log_success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Health check functions
check_api_health() {
    log_info "Checking API health..."
    
    local health_response
    health_response=$(curl -f -s --max-time "$TIMEOUT" \
        "http://${NESTGATE_HOST}:${NESTGATE_PORT}/health" 2>/dev/null || echo "FAILED")
    
    if [ "$health_response" = "FAILED" ]; then
        log_error "API health check failed"
        return 1
    fi
    
    log_success "API is healthy"
    return 0
}

check_websocket() {
    log_info "Checking WebSocket connectivity..."
    
    # Simple WebSocket connection test using curl
    if curl -f -s --max-time "$TIMEOUT" \
        -H "Upgrade: websocket" \
        -H "Connection: Upgrade" \
        "http://${NESTGATE_HOST}:8080" > /dev/null 2>&1; then
        log_success "WebSocket is accessible"
        return 0
    else
        log_warn "WebSocket connection test inconclusive"
        return 1
    fi
}

check_metrics() {
    log_info "Checking metrics endpoint..."
    
    local metrics_response
    metrics_response=$(curl -f -s --max-time "$TIMEOUT" \
        "http://${NESTGATE_HOST}:9090/metrics" 2>/dev/null || echo "FAILED")
    
    if [ "$metrics_response" = "FAILED" ]; then
        log_error "Metrics endpoint failed"
        return 1
    fi
    
    log_success "Metrics endpoint is healthy"
    return 0
}

check_storage_capacity() {
    log_info "Checking storage capacity..."
    
    local storage_info
    storage_info=$(curl -f -s --max-time "$TIMEOUT" \
        "http://${NESTGATE_HOST}:${NESTGATE_PORT}/api/v1/storage/info" 2>/dev/null || echo "FAILED")
    
    if [ "$storage_info" = "FAILED" ]; then
        log_error "Storage info check failed"
        return 1
    fi
    
    log_success "Storage information is accessible"
    return 0
}

check_docker_containers() {
    log_info "Checking Docker containers..."
    
    if ! command -v docker &> /dev/null; then
        log_warn "Docker not available for container check"
        return 1
    fi
    
    local containers
    containers=$(docker ps --filter "name=nestgate" --format "{{.Names}}" 2>/dev/null || echo "")
    
    if [ -z "$containers" ]; then
        log_error "No NestGate containers found"
        return 1
    fi
    
    for container in $containers; do
        local status
        status=$(docker inspect --format='{{.State.Status}}' "$container" 2>/dev/null || echo "unknown")
        
        if [ "$status" = "running" ]; then
            log_success "Container $container is running"
        else
            log_error "Container $container is $status"
            return 1
        fi
    done
    
    return 0
}

check_resource_usage() {
    log_info "Checking resource usage..."
    
    if ! command -v docker &> /dev/null; then
        log_warn "Docker not available for resource check"
        return 1
    fi
    
    # Get container resource usage
    local container_stats
    container_stats=$(docker stats --no-stream --format "{{.Name}},{{.CPUPerc}},{{.MemUsage}}" 2>/dev/null | grep nestgate || echo "")
    
    if [ -z "$container_stats" ]; then
        log_warn "No resource stats available"
        return 1
    fi
    
    log_success "Resource usage data available"
    if [ "$VERBOSE" = "true" ]; then
        echo "$container_stats"
    fi
    
    return 0
}

# Main health check
main() {
    echo "=============================================================================="
    echo "🔍 NestGate Health Check"
    echo "Target: ${NESTGATE_HOST}:${NESTGATE_PORT}"
    echo "Timeout: ${TIMEOUT}s"
    echo "=============================================================================="
    
    local exit_code=0
    local checks_passed=0
    local checks_total=6
    
    # Run all checks
    if check_api_health; then
        ((checks_passed++))
    else
        exit_code=1
    fi
    
    if check_websocket; then
        ((checks_passed++))
    else
        exit_code=1
    fi
    
    if check_metrics; then
        ((checks_passed++))
    else
        exit_code=1
    fi
    
    if check_storage_capacity; then
        ((checks_passed++))
    else
        exit_code=1
    fi
    
    if check_docker_containers; then
        ((checks_passed++))
    else
        exit_code=1
    fi
    
    if check_resource_usage; then
        ((checks_passed++))
    else
        exit_code=1
    fi
    
    echo "=============================================================================="
    
    if [ $exit_code -eq 0 ]; then
        log_success "All health checks passed (${checks_passed}/${checks_total})"
        echo "🎉 NestGate is healthy and ready for production use"
    else
        log_error "Health checks failed (${checks_passed}/${checks_total} passed)"
        echo "⚠️  NestGate may not be functioning correctly"
    fi
    
    echo "=============================================================================="
    
    exit $exit_code
}

# Handle command line arguments
if [ "${1:-}" = "--verbose" ] || [ "${1:-}" = "-v" ]; then
    VERBOSE=true
fi

# Run main function
main "$@" 