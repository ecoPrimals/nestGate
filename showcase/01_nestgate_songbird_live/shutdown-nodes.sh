#!/bin/bash
# Graceful shutdown of all integration services (Songbird + NestGate)

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🛑 Shutting Down Integration Services"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check what's running
echo -e "${BLUE}Checking running services...${NC}"
SONGBIRD_COUNT=$(ps aux | grep -v grep | grep songbird-orchestrator | wc -l)
NESTGATE_COUNT=$(ps aux | grep -v grep | grep "nestgate service" | wc -l)

echo "  Songbird instances: $SONGBIRD_COUNT"
echo "  NestGate instances: $NESTGATE_COUNT"
echo ""

# Shutdown Songbird
if [ $SONGBIRD_COUNT -gt 0 ]; then
    echo -e "${BLUE}Stopping Songbird orchestrator...${NC}"
    pkill -f songbird-orchestrator || true
    sleep 2
    echo -e "${GREEN}✅ Songbird stopped${NC}"
else
    echo -e "${YELLOW}No Songbird processes found${NC}"
fi
echo ""

# Shutdown NestGate
if [ $NESTGATE_COUNT -gt 0 ]; then
    echo -e "${BLUE}Stopping NestGate nodes...${NC}"
    pkill -f "nestgate service" || true
    sleep 2
    echo -e "${GREEN}✅ NestGate nodes stopped${NC}"
else
    echo -e "${YELLOW}No NestGate processes found${NC}"
fi
echo ""

# Verify shutdown
echo -e "${BLUE}Verifying shutdown...${NC}"
REMAINING=$(ps aux | grep -E '(songbird-orchestrator|nestgate service)' | grep -v grep | wc -l)

if [ $REMAINING -eq 0 ]; then
    echo -e "${GREEN}✅ All services stopped successfully${NC}"
else
    echo -e "${YELLOW}⚠️  Some processes may still be running ($REMAINING)${NC}"
    echo "   Run: ps aux | grep -E '(songbird|nestgate)'"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Shutdown Complete"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
