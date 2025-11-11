#!/bin/bash
# Start NestGate with Songbird orchestration
# Uses built-in NetworkApi for service discovery and port allocation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔════════════════════════════════════════╗"
echo "║   NESTGATE + SONGBIRD INTEGRATION     ║"
echo "╚════════════════════════════════════════╝"
echo -e "${NC}"

# Check if Songbird is running
SONGBIRD_URL="http://192.168.1.144:8080"
echo "Checking for Songbird orchestrator..."
if curl -s --connect-timeout 2 "$SONGBIRD_URL/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Songbird found at $SONGBIRD_URL"
else
    echo -e "${YELLOW}⚠${NC} Songbird not responding at $SONGBIRD_URL"
    echo "  Checking alternative endpoints..."
    
    # Try localhost
    if curl -s --connect-timeout 2 "http://localhost:8080/health" > /dev/null 2>&1; then
        SONGBIRD_URL="http://localhost:8080"
        echo -e "${GREEN}✓${NC} Songbird found at localhost:8080"
    else
        echo -e "${YELLOW}⚠${NC} Songbird not found"
        echo "  NestGate will use fallback port allocation"
    fi
fi

# Create config directory
mkdir -p ~/.nestgate/{data,logs,config,cache}

# Check if binary exists
if [ ! -f "target/release/nestgate" ]; then
    echo -e "${YELLOW}⚠ Binary not found. Building...${NC}"
    cargo build --release
fi

# Export environment for orchestration mode
export NESTGATE_ORCHESTRATION_MODE="enabled"
export NESTGATE_ORCHESTRATOR_URL="$SONGBIRD_URL"
export NESTGATE_AUTO_REGISTER="true"
export NESTGATE_SERVICE_NAME="nestgate-eastgate-dev"
export NESTGATE_CAPABILITIES="storage,zfs,dataset_management,snapshots"

echo ""
echo -e "${BLUE}Starting NestGate with Orchestration...${NC}"
echo "  Mode: OrchestrationEnhanced"
echo "  Orchestrator: $SONGBIRD_URL"
echo "  Service: nestgate-eastgate-dev"
echo "  Capabilities: storage, zfs, snapshots"
echo ""

# Start NestGate - it will:
# 1. Discover Songbird via NetworkApi
# 2. Request port allocation
# 3. Register with assigned port
# 4. Send heartbeats

echo -e "${GREEN}✓${NC} NestGate will request port from Songbird..."
echo ""

# Run with federation config
./target/release/nestgate service start \
    --config ~/.nestgate/federation-config.toml \
    --verbose &

NESTGATE_PID=$!
echo $NESTGATE_PID > ~/.nestgate/service.pid

# Wait for startup
sleep 3

# Check if started
if ps -p $NESTGATE_PID > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} NestGate started (PID: $NESTGATE_PID)"
    echo ""
    echo "Check logs:"
    echo "  tail -f ~/.nestgate/logs/nestgate.log"
    echo ""
    echo "Stop with:"
    echo "  kill $NESTGATE_PID"
else
    echo -e "${YELLOW}⚠${NC} NestGate may have failed to start"
    echo "Check logs: tail ~/.nestgate/logs/service.log"
fi

