#!/bin/bash
# Start local NestGate development instance
# Run this on Eastgate for local development and testing

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
echo "║   STARTING LOCAL NESTGATE INSTANCE    ║"
echo "╚════════════════════════════════════════╝"
echo -e "${NC}"

# Check if already running
if [ -f ~/.nestgate/service.pid ]; then
    PID=$(cat ~/.nestgate/service.pid)
    if ps -p $PID > /dev/null 2>&1; then
        echo -e "${YELLOW}⚠ NestGate already running (PID: $PID)${NC}"
        echo "Stop it first with: kill $PID"
        exit 1
    else
        # Stale PID file
        rm ~/.nestgate/service.pid
    fi
fi

# Create directories
echo "Creating directories..."
mkdir -p ~/.nestgate/{data,logs,config,cache,temp}

# Check if binary exists
if [ ! -f "target/release/nestgate" ]; then
    echo -e "${YELLOW}⚠ Binary not found. Building...${NC}"
    cargo build --release
fi

# Create default config if it doesn't exist
if [ ! -f ~/.nestgate/config.toml ]; then
    echo "Creating default config..."
    cat > ~/.nestgate/config.toml << 'EOF'
[service]
name = "nestgate-eastgate-dev"
bind_address = "127.0.0.1"
port = 8080

[storage]
data_dir = "/home/eastgate/.nestgate/data"
cache_dir = "/home/eastgate/.nestgate/cache"
temp_dir = "/home/eastgate/.nestgate/temp"

[logging]
level = "info"
dir = "/home/eastgate/.nestgate/logs"
format = "pretty"

[api]
enable_cors = true
enable_metrics = true
enable_health_checks = true

[discovery]
enabled = false
# Enable with: discovery_url = "http://localhost:9090"
EOF
    echo -e "${GREEN}✓${NC} Created ~/.nestgate/config.toml"
fi

# Start service
echo -e "\n${BLUE}Starting NestGate service...${NC}"
./target/release/nestgate service start \
    --config ~/.nestgate/config.toml \
    > ~/.nestgate/logs/service.log 2>&1 &

PID=$!
echo $PID > ~/.nestgate/service.pid

# Wait for startup
echo "Waiting for service to start..."
for i in {1..10}; do
    if curl -s http://127.0.0.1:8080/health > /dev/null 2>&1; then
        break
    fi
    if ! ps -p $PID > /dev/null 2>&1; then
        echo -e "${YELLOW}⚠ Service failed to start${NC}"
        echo "Check logs: tail ~/.nestgate/logs/service.log"
        rm ~/.nestgate/service.pid
        exit 1
    fi
    sleep 1
done

# Verify running
if curl -s http://127.0.0.1:8080/health > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} NestGate started successfully (PID: $PID)"
else
    echo -e "${YELLOW}⚠ Service may not be fully ready${NC}"
fi

# Display info
echo ""
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${GREEN}Local NestGate Instance Running${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo ""
echo "  API URL:    http://127.0.0.1:8080"
echo "  Health:     http://127.0.0.1:8080/health"
echo "  Config:     ~/.nestgate/config.toml"
echo "  Logs:       ~/.nestgate/logs/service.log"
echo "  PID:        $PID"
echo ""
echo "Test with:"
echo "  export NESTGATE_API_URL='http://127.0.0.1:8080'"
echo "  ./target/release/nestgate version"
echo "  ./target/release/nestgate status"
echo ""
echo "Stop with:"
echo "  kill $PID"
echo "  # or run: ./stop_local_dev.sh"
echo ""
echo "View logs:"
echo "  tail -f ~/.nestgate/logs/service.log"
echo ""

# Check for Songbird
if curl -s http://localhost:9090/health > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Songbird detected at localhost:9090"
    echo "  Enable discovery in ~/.nestgate/config.toml"
else
    echo -e "${YELLOW}ℹ${NC} Songbird not detected (optional)"
    echo "  Start with: cd ../songbird && ./start_service.sh"
fi

echo ""
echo -e "${GREEN}✓ Ready for development!${NC}"

