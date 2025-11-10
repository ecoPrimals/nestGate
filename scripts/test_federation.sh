#!/bin/bash
# ==============================================================================
# Test NestGate Federation Integration
# ==============================================================================

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${BLUE}▶ $1${NC}"; }
log_success() { echo -e "${GREEN}✓ $1${NC}"; }

HOST="192.168.1.144"

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║      🧪 FEDERATION INTEGRATION TEST                       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Test 1: Check running services
log_info "Test 1: Checking federation members..."
echo ""

if curl -s http://$HOST:8080/health > /dev/null 2>&1; then
    log_success "songbird orchestrator: ONLINE (port 8080)"
else
    echo "⚠️  songbird: OFFLINE"
fi

if curl -s http://$HOST:8084/health > /dev/null 2>&1; then
    log_success "toadstool BYOB server: ONLINE (port 8084)"
else
    echo "⚠️  toadstool: OFFLINE"
fi

if curl -s http://$HOST:9001/status > /dev/null 2>&1; then
    log_success "NestGate storage: ONLINE (port 9001)"
    NESTGATE_RUNNING=true
else
    echo "⚠️  NestGate: OFFLINE (start with ./scripts/start_federation_service.sh)"
    NESTGATE_RUNNING=false
fi

echo ""

# Test 2: Check NestGate health
if [ "$NESTGATE_RUNNING" = true ]; then
    log_info "Test 2: NestGate health check..."
    curl -s http://$HOST:9002/health | jq '.'
    echo ""
fi

# Test 3: Check capabilities
if [ "$NESTGATE_RUNNING" = true ]; then
    log_info "Test 3: NestGate capabilities..."
    curl -s http://$HOST:9001/api/v1/capabilities | jq '.capabilities'
    echo ""
fi

# Test 4: List datasets
if [ "$NESTGATE_RUNNING" = true ]; then
    log_info "Test 4: Available datasets..."
    curl -s http://$HOST:9001/api/v1/datasets | jq '.datasets[] | {id, primal, size_gb, type}'
    echo ""
fi

# Test 5: Check federation status
if [ "$NESTGATE_RUNNING" = true ]; then
    log_info "Test 5: Federation status..."
    curl -s http://$HOST:9001/status | jq '{service_name, version, status, federation_members}'
    echo ""
fi

echo ""
log_success "Federation test complete!"
echo ""

if [ "$NESTGATE_RUNNING" = false ]; then
    echo "To start NestGate federation service:"
    echo "  cd /home/eastgate/Development/ecoPrimals/nestgate"
    echo "  ./scripts/start_federation_service.sh"
    echo ""
fi

exit 0

