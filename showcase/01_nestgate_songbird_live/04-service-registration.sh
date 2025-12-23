#!/bin/bash
# Phase 3: Service Registration Demo
# Register NestGate nodes with Songbird orchestrator

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║                                                                   ║"
echo "║  📝 Phase 3: Service Registration Demo                          ║"
echo "║                                                                   ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

SONGBIRD_URL="http://localhost:8080"
REGISTRATION_API="$SONGBIRD_URL/api/v1/services/register"

echo -e "${BLUE}🎯 Registration Flow:${NC}"
echo ""
echo "  1. Verify Songbird is running"
echo "  2. Register Westgate (9005)"
echo "  3. Register Eastgate (9006)"
echo "  4. Verify registrations"
echo "  5. Query registered services"
echo ""

# Step 1: Verify Songbird
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}📡 Step 1: Verifying Songbird Orchestrator${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if ! ps aux | grep -v grep | grep songbird-orchestrator > /dev/null; then
    echo -e "${RED}❌ Songbird is not running${NC}"
    echo "Please start it first:"
    echo "  ./03-live-integration.sh"
    exit 1
fi

echo -e "${GREEN}✅ Songbird is running${NC}"

# Check if API is responding
if curl -s --max-time 2 "$SONGBIRD_URL/api/v1/info" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Songbird API is responding${NC}"
    INFO=$(curl -s "$SONGBIRD_URL/api/v1/info" 2>/dev/null)
    echo "   Orchestrator: $(echo "$INFO" | jq -r '.name' 2>/dev/null || echo 'Songbird')"
else
    echo -e "${YELLOW}⚠️  API endpoint not responding (may be using different routes)${NC}"
fi
echo ""

# Step 2: Register Westgate
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}📝 Step 2: Registering Westgate (Port 9005)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Verify Westgate is running
if ! curl -s http://localhost:9005/health > /dev/null 2>&1; then
    echo -e "${RED}❌ Westgate is not running on port 9005${NC}"
    exit 1
fi

echo -e "${CYAN}Preparing registration request...${NC}"

# Registration payload
WESTGATE_REGISTRATION=$(cat <<EOF
{
  "primal_name": "nestgate-westgate",
  "version": "0.1.0",
  "capabilities": [
    {
      "name": "storage",
      "capability_type": "Storage",
      "metadata": {
        "features": ["zfs", "snapshots", "compression", "deduplication"],
        "max_capacity_gb": 1000
      }
    },
    {
      "name": "replication",
      "capability_type": "Coordination",
      "metadata": {
        "protocol": "http"
      }
    }
  ],
  "protocols": ["http", "websocket"],
  "preferred_protocol": "http",
  "endpoint": {
    "protocol": "http",
    "host": "localhost",
    "port": 9005,
    "full_url": "http://localhost:9005"
  },
  "metadata": {
    "node_name": "westgate",
    "location": "local",
    "health_endpoint": "http://localhost:9005/health"
  }
}
EOF
)

echo "Sending registration to Songbird..."
WEST_RESPONSE=$(curl -s -X POST "$REGISTRATION_API" \
  -H "Content-Type: application/json" \
  -d "$WESTGATE_REGISTRATION" 2>&1)

echo ""
if echo "$WEST_RESPONSE" | jq -e '.service_id' > /dev/null 2>&1; then
    WEST_ID=$(echo "$WEST_RESPONSE" | jq -r '.service_id')
    echo -e "${GREEN}✅ Westgate registered successfully${NC}"
    echo "   Service ID: $WEST_ID"
    echo "   Status: $(echo "$WEST_RESPONSE" | jq -r '.status' 2>/dev/null || echo 'registered')"
    echo "   Token: $(echo "$WEST_RESPONSE" | jq -r '.token' 2>/dev/null | cut -c1-20)..."
else
    echo -e "${YELLOW}⚠️  Registration response: $WEST_RESPONSE${NC}"
    echo "   (Endpoint may not be implemented yet - this is expected)"
fi
echo ""

# Step 3: Register Eastgate
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}📝 Step 3: Registering Eastgate (Port 9006)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Verify Eastgate is running
if ! curl -s http://localhost:9006/health > /dev/null 2>&1; then
    echo -e "${RED}❌ Eastgate is not running on port 9006${NC}"
    exit 1
fi

EASTGATE_REGISTRATION=$(cat <<EOF
{
  "primal_name": "nestgate-eastgate",
  "version": "0.1.0",
  "capabilities": [
    {
      "name": "storage",
      "capability_type": "Storage",
      "metadata": {
        "features": ["zfs", "snapshots", "compression", "deduplication"],
        "max_capacity_gb": 1000
      }
    },
    {
      "name": "replication",
      "capability_type": "Coordination",
      "metadata": {
        "protocol": "http"
      }
    }
  ],
  "protocols": ["http", "websocket"],
  "preferred_protocol": "http",
  "endpoint": {
    "protocol": "http",
    "host": "localhost",
    "port": 9006,
    "full_url": "http://localhost:9006"
  },
  "metadata": {
    "node_name": "eastgate",
    "location": "local",
    "health_endpoint": "http://localhost:9006/health"
  }
}
EOF
)

echo "Sending registration to Songbird..."
EAST_RESPONSE=$(curl -s -X POST "$REGISTRATION_API" \
  -H "Content-Type: application/json" \
  -d "$EASTGATE_REGISTRATION" 2>&1)

echo ""
if echo "$EAST_RESPONSE" | jq -e '.service_id' > /dev/null 2>&1; then
    EAST_ID=$(echo "$EAST_RESPONSE" | jq -r '.service_id')
    echo -e "${GREEN}✅ Eastgate registered successfully${NC}"
    echo "   Service ID: $EAST_ID"
    echo "   Status: $(echo "$EAST_RESPONSE" | jq -r '.status' 2>/dev/null || echo 'registered')"
    echo "   Token: $(echo "$EAST_RESPONSE" | jq -r '.token' 2>/dev/null | cut -c1-20)..."
else
    echo -e "${YELLOW}⚠️  Registration response: $EAST_RESPONSE${NC}"
    echo "   (Endpoint may not be implemented yet - this is expected)"
fi
echo ""

# Step 4: Query Services
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}🔍 Step 4: Querying Registered Services${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Querying all registered services..."
SERVICES=$(curl -s "$SONGBIRD_URL/api/v1/services" 2>&1)

if echo "$SERVICES" | jq -e '.' > /dev/null 2>&1; then
    SERVICE_COUNT=$(echo "$SERVICES" | jq '. | length' 2>/dev/null || echo "0")
    echo -e "${GREEN}✅ Services endpoint responding${NC}"
    echo "   Registered services: $SERVICE_COUNT"
    echo ""
    echo "Services:"
    echo "$SERVICES" | jq '.' 2>/dev/null || echo "$SERVICES"
else
    echo -e "${YELLOW}⚠️  Services list: $SERVICES${NC}"
    echo "   (Endpoint may not be implemented yet)"
fi
echo ""

# Query by capability
echo "Querying storage capability..."
STORAGE_SERVICES=$(curl -s "$SONGBIRD_URL/api/v1/services/query/storage" 2>&1)

if echo "$STORAGE_SERVICES" | jq -e '.' > /dev/null 2>&1; then
    STORAGE_COUNT=$(echo "$STORAGE_SERVICES" | jq '. | length' 2>/dev/null || echo "0")
    echo -e "${GREEN}✅ Storage capability query responded${NC}"
    echo "   Storage services: $STORAGE_COUNT"
else
    echo -e "${YELLOW}⚠️  Storage query response: $STORAGE_SERVICES${NC}"
fi
echo ""

# Step 5: Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✅ Service Registration Demo Complete!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "📊 Summary:"
echo "   • Songbird orchestrator: Running ✅"
echo "   • Westgate registration: Attempted ✅"
echo "   • Eastgate registration: Attempted ✅"
echo "   • Service queries: Attempted ✅"
echo ""

echo "🎓 What This Demonstrates:"
echo "   • Service registration protocol"
echo "   • Capability-based discovery"
echo "   • NestGate metadata sharing"
echo "   • Foundation for coordination"
echo ""

echo "💡 Notes:"
echo "   • If registration endpoints returned errors, that's expected"
echo "   • This shows the PROTOCOL even if not fully implemented"
echo "   • Songbird may be running without full service registry"
echo "   • The pattern is established for future implementation"
echo ""

echo "⏭️  Next Steps:"
echo "   1. Check if Songbird has service registry enabled"
echo "   2. Implement NestGate-side registration client"
echo "   3. Add heartbeat mechanism"
echo "   4. Build coordinated operations"
echo ""

echo "🎯 Achievement: Demonstrated service registration protocol!"

