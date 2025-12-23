#!/bin/bash
# Verify Live Integration Status

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║   🔍 Live Integration Status Check                               ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Check Songbird
echo -e "${BLUE}🎵 Songbird Orchestrator:${NC}"
if ps aux | grep -v grep | grep songbird-orchestrator > /dev/null; then
    PID=$(ps aux | grep -v grep | grep songbird-orchestrator | awk '{print $2}')
    echo -e "${GREEN}  ✅ Running (PID: $PID)${NC}"
    echo "     Port: 8080"
else
    echo -e "${RED}  ❌ Not running${NC}"
fi
echo ""

# Check NestGate nodes
echo -e "${BLUE}🏗️  NestGate Nodes:${NC}"
for port in 9005 9006; do
    if curl -s --max-time 1 http://localhost:$port/health > /dev/null 2>&1; then
        HEALTH=$(curl -s http://localhost:$port/health)
        SERVICE=$(echo "$HEALTH" | jq -r '.service' 2>/dev/null || echo "nestgate")
        STATUS=$(echo "$HEALTH" | jq -r '.status' 2>/dev/null || echo "ok")
        echo -e "${GREEN}  ✅ Port $port: $SERVICE ($STATUS)${NC}"
    else
        echo -e "${RED}  ❌ Port $port: Not responding${NC}"
    fi
done
echo ""

# Process count
echo -e "${BLUE}📊 Process Summary:${NC}"
SONGBIRD_COUNT=$(ps aux | grep -v grep | grep songbird-orchestrator | wc -l)
NESTGATE_COUNT=$(ps aux | grep -v grep | grep "nestgate service" | wc -l)
echo "  Songbird instances: $SONGBIRD_COUNT"
echo "  NestGate instances: $NESTGATE_COUNT"
echo "  Total: $((SONGBIRD_COUNT + NESTGATE_COUNT))"
echo ""

# Architecture diagram
echo -e "${BLUE}🎯 Current Architecture:${NC}"
echo ""
if [ $SONGBIRD_COUNT -gt 0 ]; then
    echo "  ┌─────────────────────────────────┐"
    echo "  │   Songbird :8080 ✅             │"
    echo "  └────────┬────────────────────────┘"
else
    echo "  ┌─────────────────────────────────┐"
    echo "  │   Songbird :8080 ❌             │"
    echo "  └────────┬────────────────────────┘"
fi
echo "           │"
echo "      ┌────┴────┬──────────────┐"
echo "      │         │              │"

# Check each node
NODE1="❌"
NODE2="❌"
NODE3="❌"
curl -s --max-time 1 http://localhost:9005/health > /dev/null 2>&1 && NODE1="✅"
curl -s --max-time 1 http://localhost:9006/health > /dev/null 2>&1 && NODE2="✅"
curl -s --max-time 1 http://localhost:9007/health > /dev/null 2>&1 && NODE3="✅"

echo "  ┌───────┐ ┌────────┐ ┌─────────┐"
echo "  │West $NODE1│ │East $NODE2 │ │North $NODE3 │"
echo "  │ 9005  │ │ 9006   │ │ 9007    │"
echo "  └───────┘ └────────┘ └─────────┘"
echo ""

# Status summary
ACTIVE=0
[ "$SONGBIRD_COUNT" -gt 0 ] && ACTIVE=$((ACTIVE + 1))
curl -s --max-time 1 http://localhost:9005/health > /dev/null 2>&1 && ACTIVE=$((ACTIVE + 1))
curl -s --max-time 1 http://localhost:9006/health > /dev/null 2>&1 && ACTIVE=$((ACTIVE + 1))

echo -e "${BLUE}📈 Integration Status:${NC}"
if [ $ACTIVE -ge 3 ]; then
    echo -e "${GREEN}  ✅ EXCELLENT: Full integration running ($ACTIVE/3+ services)${NC}"
elif [ $ACTIVE -ge 2 ]; then
    echo -e "${GREEN}  ✅ GOOD: Partial integration running ($ACTIVE/3+ services)${NC}"
elif [ $ACTIVE -ge 1 ]; then
    echo -e "${YELLOW}  ⚠️  LIMITED: Some services running ($ACTIVE/3+ services)${NC}"
else
    echo -e "${RED}  ❌ NONE: No services running${NC}"
fi
echo ""

# Quick commands
echo -e "${BLUE}💡 Quick Commands:${NC}"
echo "  Check Songbird: ps aux | grep songbird-orchestrator"
echo "  Check NestGate: ps aux | grep 'nestgate service'"
echo "  Health checks:  curl http://localhost:{9005,9006}/health | jq"
echo "  Stop all:       pkill -f 'songbird-orchestrator|nestgate service'"
echo ""

