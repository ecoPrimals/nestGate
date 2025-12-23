#!/usr/bin/env bash
# NestGate Showcase - Quick Start
# Runs the fastest path to seeing NestGate in action

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                                                            ║"
echo "║          NestGate Showcase - QUICK START                  ║"
echo "║                                                            ║"
echo "║  This will:                                                ║"
echo "║  1. Check prerequisites                                    ║"
echo "║  2. Build NestGate (if needed)                            ║"
echo "║  3. Start NestGate service                                ║"
echo "║  4. Run first demo (Storage Basics)                       ║"
echo "║                                                            ║"
echo "║  Estimated time: 5 minutes                                ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Step 1: Check prerequisites
echo -e "${BLUE}Step 1/4:${NC} Checking prerequisites..."
if ! ./utils/setup/check_prerequisites.sh; then
    echo ""
    echo -e "${YELLOW}Some prerequisites are missing.${NC}"
    echo "Please install required tools and try again."
    exit 1
fi

echo ""
echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo ""

# Step 2: Build NestGate if needed
echo -e "${BLUE}Step 2/4:${NC} Checking NestGate build..."
NESTGATE_ROOT="$(cd ../.. && pwd)"

if [ ! -f "$NESTGATE_ROOT/target/release/nestgate" ]; then
    echo "NestGate not built yet. Building now..."
    echo "This may take 5-10 minutes on first run..."
    cd "$NESTGATE_ROOT"
    cargo build --release
    cd - > /dev/null
    echo -e "${GREEN}✓ Build complete${NC}"
else
    echo -e "${GREEN}✓ NestGate already built${NC}"
fi

echo ""

# Step 3: Start NestGate
echo -e "${BLUE}Step 3/4:${NC} Starting NestGate service..."

# Check if already running
if curl -s -f http://localhost:8080/health > /dev/null 2>&1; then
    echo -e "${GREEN}✓ NestGate already running${NC}"
else
    echo "Starting NestGate..."
    if [ -f "$NESTGATE_ROOT/showcase/scripts/start_data_service.sh" ]; then
        "$NESTGATE_ROOT/showcase/scripts/start_data_service.sh"
    else
        # Fallback: start directly
        "$NESTGATE_ROOT/target/release/nestgate" &
        NESTGATE_PID=$!
        echo "Started NestGate (PID: $NESTGATE_PID)"
        
        # Wait for startup
        echo "Waiting for NestGate to start..."
        for i in {1..30}; do
            if curl -s -f http://localhost:8080/health > /dev/null 2>&1; then
                break
            fi
            sleep 1
            echo -n "."
        done
        echo ""
    fi
    
    # Verify started
    if curl -s -f http://localhost:8080/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓ NestGate started successfully${NC}"
    else
        echo -e "${YELLOW}⚠ Could not verify NestGate started${NC}"
        echo "Continuing anyway..."
    fi
fi

echo ""

# Step 4: Run first demo
echo -e "${BLUE}Step 4/4:${NC} Running first demo (Storage Basics)..."
echo ""
echo "═══════════════════════════════════════════════════════════"
echo ""

cd 01_isolated/01_storage_basics
./demo.sh

echo ""
echo "═══════════════════════════════════════════════════════════"
echo ""
echo -e "${GREEN}✓ Quick Start Complete!${NC}"
echo ""
echo "What you just saw:"
echo "  • NestGate storing and retrieving data"
echo "  • REST API operations"
echo "  • Dataset management"
echo ""
echo "What's next:"
echo "  1. Run more Level 1 demos:"
echo "     cd 01_isolated && ls -d */"
echo ""
echo "  2. Try Level 2 (Ecosystem Integration):"
echo "     cd ../02_ecosystem_integration"
echo ""
echo "  3. Read the guide:"
echo "     cat ../00_START_HERE.md"
echo ""
echo "  4. Stop NestGate when done:"
echo "     curl -X POST http://localhost:8080/admin/shutdown"
echo ""

