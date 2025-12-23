#!/bin/bash
# Live Demo 2A.1: BearDog Discovery & Fallback
# 
# Demonstrates:
# - Runtime discovery of BearDog
# - Graceful fallback if unavailable
# - Zero hardcoded dependencies

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🌍 Live Demo 2A.1: BearDog Discovery & Fallback${NC}"
echo "================================================"
echo ""

# Get project root (nestgate root)
PROJECT_ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"

# Run the live integration example
echo -e "${BLUE}Running live integration demo...${NC}"
echo ""

cd "$PROJECT_ROOT"
cargo run --example live-integration-01-storage-security

echo ""
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo ""
echo -e "${BLUE}What you just saw:${NC}"
echo "  1. Runtime discovery attempted"
echo "  2. Health check performed"
echo "  3. Integration or fallback demonstrated"
echo "  4. Zero hardcoding verified"
echo ""
echo -e "${BLUE}To see full integration:${NC}"
echo "  1. Start BearDog: cd ../beardog && ./target/release/examples/btsp_server"
echo "  2. Run this demo again"
echo "  3. Observe enhanced integration"
echo ""
echo -e "${BLUE}Learn more:${NC}"
echo "  - README.md (this directory)"
echo "  - /README_ECOSYSTEM_INTEGRATION.md"
echo "  - /00_ECOSYSTEM_INTEGRATION_COMPLETE_DEC_21_2025.md"

