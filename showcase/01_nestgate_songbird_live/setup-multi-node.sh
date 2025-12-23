#!/bin/bash
# Setup Multi-Node NestGate with Songbird Orchestration
# This creates a live federation of NestGate storage nodes

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🏗️  NestGate Multi-Node Setup with Songbird"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Configuration
NESTGATE_BIN="/home/eastgate/Development/ecoPrimals/nestgate/target/release/nestgate"
SONGBIRD_URL="http://localhost:8080"
OUTPUT_DIR="outputs/multi-node-$(date +%s)"
mkdir -p "$OUTPUT_DIR"

# Generate JWT secret
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)
echo -e "${GREEN}✅ Generated JWT secret for demo${NC}"
echo ""

# Binary check - try both possible locations
if ! [ -f "$NESTGATE_BIN" ]; then
    # Try debug build if release not found
    NESTGATE_BIN="/home/eastgate/Development/ecoPrimals/nestgate/target/debug/nestgate"
fi

# Check prerequisites
echo -e "${BLUE}🔍 Checking prerequisites...${NC}"
echo ""

if ! [ -f "$NESTGATE_BIN" ]; then
    echo -e "${RED}❌ NestGate binary not found!${NC}"
    echo "Please build it first:"
    echo "  cd /home/eastgate/Development/ecoPrimals/nestgate"
    echo "  cargo build --release --bin nestgate"
    exit 1
fi
echo -e "${GREEN}✅ NestGate binary found${NC}"

if ! curl -s "$SONGBIRD_URL/health" > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Songbird not responding at $SONGBIRD_URL${NC}"
    echo "This demo will run in standalone mode"
    STANDALONE=true
else
    echo -e "${GREEN}✅ Songbird is running${NC}"
    STANDALONE=false
fi

echo ""

# Kill any existing NestGate processes
echo -e "${BLUE}🧹 Cleaning up existing processes...${NC}"
pkill -f "nestgate" || true
sleep 2
echo -e "${GREEN}✅ Cleanup complete${NC}"
echo ""

# Create data directories
echo -e "${BLUE}📁 Creating data directories...${NC}"
mkdir -p /tmp/nestgate-west
mkdir -p /tmp/nestgate-east
mkdir -p /tmp/nestgate-north
echo -e "${GREEN}✅ Directories created${NC}"
echo ""

# Start Node 1: Westgate
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}🚀 Starting Node 1: Westgate (Port 9005)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

NESTGATE_API_PORT=9005 NESTGATE_DATA_DIR=/tmp/nestgate-west \
  NESTGATE_JWT_SECRET="$NESTGATE_JWT_SECRET" \
  "$NESTGATE_BIN" service start --port 9005 \
  > "$OUTPUT_DIR/westgate.log" 2>&1 &

WEST_PID=$!
echo $WEST_PID > "$OUTPUT_DIR/westgate.pid"
sleep 3

if ps -p $WEST_PID > /dev/null; then
    echo -e "${GREEN}✅ Westgate started (PID: $WEST_PID)${NC}"
else
    echo -e "${RED}❌ Failed to start Westgate${NC}"
    cat "$OUTPUT_DIR/westgate.log"
    exit 1
fi
echo ""

# Start Node 2: Eastgate
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}🚀 Starting Node 2: Eastgate (Port 9006)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

NESTGATE_API_PORT=9006 NESTGATE_DATA_DIR=/tmp/nestgate-east \
  NESTGATE_JWT_SECRET="$NESTGATE_JWT_SECRET" \
  "$NESTGATE_BIN" service start --port 9006 \
  > "$OUTPUT_DIR/eastgate.log" 2>&1 &

EAST_PID=$!
echo $EAST_PID > "$OUTPUT_DIR/eastgate.pid"
sleep 3

if ps -p $EAST_PID > /dev/null; then
    echo -e "${GREEN}✅ Eastgate started (PID: $EAST_PID)${NC}"
else
    echo -e "${RED}❌ Failed to start Eastgate${NC}"
    cat "$OUTPUT_DIR/eastgate.log"
    exit 1
fi
echo ""

# Start Node 3: Northgate
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}🚀 Starting Node 3: Northgate (Port 9007)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

NESTGATE_API_PORT=9007 NESTGATE_DATA_DIR=/tmp/nestgate-north \
  NESTGATE_JWT_SECRET="$NESTGATE_JWT_SECRET" \
  "$NESTGATE_BIN" service start --port 9007 \
  > "$OUTPUT_DIR/northgate.log" 2>&1 &

NORTH_PID=$!
echo $NORTH_PID > "$OUTPUT_DIR/northgate.pid"
sleep 3

if ps -p $NORTH_PID > /dev/null; then
    echo -e "${GREEN}✅ Northgate started (PID: $NORTH_PID)${NC}"
else
    echo -e "${RED}❌ Failed to start Northgate${NC}"
    cat "$OUTPUT_DIR/northgate.log"
    exit 1
fi
echo ""

# Verify all nodes are responding
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}🔍 Verifying node health...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

sleep 3

for port in 9005 9006 9007; do
    if curl -s http://localhost:$port/health > /dev/null 2>&1; then
        name=$(curl -s http://localhost:$port/health | jq -r '.name // "unknown"')
        echo -e "${GREEN}✅ Port $port: $name responding${NC}"
    else
        echo -e "${RED}❌ Port $port: Not responding${NC}"
    fi
done
echo ""

# Register with Songbird (if available)
if [ "$STANDALONE" = false ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${BLUE}📡 Registering nodes with Songbird...${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Register each node
    for node in "westgate:9005" "eastgate:9006" "northgate:9007"; do
        name="${node%:*}"
        port="${node#*:}"
        
        echo "Registering $name..."
        response=$(curl -s -X POST "$SONGBIRD_URL/api/federation/register" \
          -H "Content-Type: application/json" \
          -d "{
            \"name\": \"$name\",
            \"service_type\": \"storage\",
            \"endpoint\": \"http://localhost:$port\",
            \"capabilities\": [\"storage\", \"replication\", \"snapshots\"],
            \"health_endpoint\": \"http://localhost:$port/health\"
          }" 2>&1)
        
        if echo "$response" | grep -q "success\|registered\|ok" 2>/dev/null; then
            echo -e "${GREEN}✅ $name registered${NC}"
        else
            echo -e "${YELLOW}⚠️  $name registration: $response${NC}"
        fi
    done
    echo ""
fi

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✅ Multi-Node Setup Complete!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Status:"
echo "   Westgate:  http://localhost:9005 (PID: $WEST_PID)"
echo "   Eastgate:  http://localhost:9006 (PID: $EAST_PID)"
echo "   Northgate: http://localhost:9007 (PID: $NORTH_PID)"
echo ""
echo "📁 Logs: $OUTPUT_DIR/"
echo ""

if [ "$STANDALONE" = false ]; then
    echo "🎯 Songbird Integration:"
    echo "   View services: curl $SONGBIRD_URL/api/federation/services?type=storage | jq"
    echo ""
fi

echo "⏭️  Next Steps:"
echo "   1. Run coordinated operations:"
echo "      ./02-coordinated-storage.sh"
echo ""
echo "   2. Test load balancing:"
echo "      ./03-load-balancing.sh"
echo ""
echo "   3. Demonstrate failover:"
echo "      ./04-failover-demo.sh"
echo ""
echo "💡 To stop all nodes:"
echo "   ./shutdown-nodes.sh"
echo ""

# Save environment info
cat > "$OUTPUT_DIR/environment.json" << EOF
{
  "timestamp": "$(date -Iseconds)",
  "nodes": [
    {"name": "westgate", "port": 9005, "pid": $WEST_PID, "data_dir": "/tmp/nestgate-west"},
    {"name": "eastgate", "port": 9006, "pid": $EAST_PID, "data_dir": "/tmp/nestgate-east"},
    {"name": "northgate", "port": 9007, "pid": $NORTH_PID, "data_dir": "/tmp/nestgate-north"}
  ],
  "songbird": {
    "available": $([ "$STANDALONE" = false ] && echo "true" || echo "false"),
    "url": "$SONGBIRD_URL"
  },
  "output_dir": "$OUTPUT_DIR"
}
EOF

echo "🎉 Ready for multi-node demonstrations!"
