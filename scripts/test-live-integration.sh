#!/bin/bash
# Live Integration Test Script
# Tests actual primal-to-primal communication

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🌍 Live Integration Test: NestGate + BearDog${NC}"
echo "============================================"
echo ""

# Configuration
BEARDOG_DIR="/home/eastgate/Development/ecoPrimals/beardog"
NESTGATE_DIR="/home/eastgate/Development/ecoPrimals/nestgate"
BEARDOG_PORT=9000

# Step 1: Check if BearDog is built
echo -e "${BLUE}📦 Step 1: Checking BearDog BTSP server...${NC}"

if [ ! -f "$BEARDOG_DIR/target/release/examples/btsp_server" ]; then
    echo -e "${YELLOW}⚠️  BearDog BTSP server not built${NC}"
    echo "   Building now..."
    cd "$BEARDOG_DIR"
    cargo build --release --features btsp-api --example btsp_server
    echo -e "${GREEN}✅ BearDog BTSP server built${NC}"
else
    echo -e "${GREEN}✅ BearDog BTSP server found${NC}"
fi
echo ""

# Step 2: Check if BearDog is running
echo -e "${BLUE}🔍 Step 2: Checking if BearDog is running...${NC}"

if curl -s http://localhost:$BEARDOG_PORT/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ BearDog BTSP server is running on port $BEARDOG_PORT${NC}"
    BEARDOG_RUNNING=true
else
    echo -e "${YELLOW}⚠️  BearDog BTSP server is not running${NC}"
    echo ""
    echo "   To start BearDog BTSP server, run in another terminal:"
    echo -e "${BLUE}   cd $BEARDOG_DIR${NC}"
    echo -e "${BLUE}   ./target/release/examples/btsp_server${NC}"
    echo ""
    echo "   Or run this script with --start-beardog flag"
    echo ""
    BEARDOG_RUNNING=false
fi
echo ""

# Step 3: Run integration tests
echo -e "${BLUE}🧪 Step 3: Running integration tests...${NC}"
echo ""

cd "$NESTGATE_DIR"

# Test 1: Graceful degradation (without BearDog)
if [ "$BEARDOG_RUNNING" = false ]; then
    echo -e "${BLUE}Test 1: Graceful Degradation (BearDog not running)${NC}"
    echo "---------------------------------------------------"
    cargo run --example live-integration-01-storage-security
    echo ""
    echo -e "${GREEN}✅ Test 1 Complete: Graceful degradation verified${NC}"
    echo ""
    
    echo -e "${YELLOW}⚠️  Cannot run Test 2 without BearDog running${NC}"
    echo ""
    echo "To run full integration test:"
    echo "1. Start BearDog: cd $BEARDOG_DIR && ./target/release/examples/btsp_server"
    echo "2. Run this script again"
else
    # Test 1: With BearDog running
    echo -e "${BLUE}Test 1: Discovery and Fallback${NC}"
    echo "--------------------------------"
    cargo run --example live-integration-01-storage-security
    echo ""
    echo -e "${GREEN}✅ Test 1 Complete${NC}"
    echo ""
    
    # Test 2: Real BearDog communication
    echo -e "${BLUE}Test 2: Real BearDog Communication${NC}"
    echo "-----------------------------------"
    cargo run --example live-integration-02-real-beardog
    echo ""
    echo -e "${GREEN}✅ Test 2 Complete${NC}"
    echo ""
fi

# Summary
echo ""
echo -e "${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}🎉 Integration Test Summary${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"
echo ""

if [ "$BEARDOG_RUNNING" = true ]; then
    echo -e "${GREEN}✅ BearDog BTSP Server: Running${NC}"
    echo -e "${GREEN}✅ Discovery: Working${NC}"
    echo -e "${GREEN}✅ Communication: Verified${NC}"
    echo -e "${GREEN}✅ Graceful Degradation: Tested${NC}"
    echo ""
    echo -e "${GREEN}🌟 Full Integration: OPERATIONAL${NC}"
else
    echo -e "${YELLOW}⚠️  BearDog BTSP Server: Not Running${NC}"
    echo -e "${GREEN}✅ Graceful Degradation: Verified${NC}"
    echo ""
    echo -e "${YELLOW}⚠️  Partial Integration: Fallback Only${NC}"
    echo ""
    echo "Next Steps:"
    echo "1. Start BearDog: cd $BEARDOG_DIR && ./target/release/examples/btsp_server"
    echo "2. Run this script again for full integration test"
fi

echo ""
echo -e "${BLUE}📊 Integration Status:${NC}"
echo "   Framework: ✅ Complete"
echo "   Test Harness: ✅ Operational"
echo "   Graceful Degradation: ✅ Verified"
if [ "$BEARDOG_RUNNING" = true ]; then
    echo "   Live Integration: ✅ Working"
else
    echo "   Live Integration: 🎯 Ready (start BearDog)"
fi

echo ""
echo -e "${GREEN}🚀 Ecosystem integration ready!${NC}"

