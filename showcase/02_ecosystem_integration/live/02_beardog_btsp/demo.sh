#!/bin/bash
# Live Demo 2A.2: BearDog BTSP Communication
# 
# Demonstrates:
# - BTSP server discovery
# - HTTP API communication
# - API structure exploration

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🌍 Live Demo 2A.2: BearDog BTSP Communication${NC}"
echo "==================================================="
echo ""

# Check if BearDog is running
echo -e "${BLUE}Checking if BearDog BTSP server is running...${NC}"
if curl -s http://localhost:9000/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ BearDog BTSP server is running${NC}"
    echo ""
else
    echo -e "${RED}❌ BearDog BTSP server is NOT running${NC}"
    echo ""
    echo -e "${YELLOW}To start BearDog BTSP server:${NC}"
    echo "  cd /home/eastgate/Development/ecoPrimals/beardog"
    echo "  ./target/release/examples/btsp_server"
    echo ""
    echo -e "${YELLOW}Then run this demo again.${NC}"
    exit 1
fi

# Get project root (nestgate root)
PROJECT_ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"

# Run the live integration example
echo -e "${BLUE}Running BTSP communication demo...${NC}"
echo ""

cd "$PROJECT_ROOT"
cargo run --example live-integration-02-real-beardog

echo ""
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo ""
echo -e "${BLUE}What you just saw:${NC}"
echo "  1. Health check communication"
echo "  2. BTSP API discovery"
echo "  3. Request/response handling"
echo "  4. Error message parsing"
echo ""
echo -e "${BLUE}API Discovered:${NC}"
echo "  - Health: GET /health"
echo "  - Tunnel: POST /btsp/tunnel/establish"
echo "  - Encrypt: POST /btsp/tunnel/encrypt"
echo "  - Decrypt: POST /btsp/tunnel/decrypt"
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo "  1. Implement correct request format"
echo "  2. Test full encryption workflow"
echo "  3. See ../03_encrypted_storage/demo.sh"
echo ""
echo -e "${BLUE}Learn more:${NC}"
echo "  - README.md (this directory)"
echo "  - /LIVE_INTEGRATION_SUCCESS_DEC_21_2025.md"

