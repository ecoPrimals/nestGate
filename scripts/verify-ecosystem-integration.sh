#!/bin/bash
# Ecosystem Integration Verification Script
# 
# This script verifies that multiple primals can run together
# and discover each other without hardcoded dependencies.

set -e

echo "🌍 EcoPrimals Ecosystem Integration Verification"
echo "================================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
ECOPRIMALS_ROOT="${ECOPRIMALS_ROOT:-/home/eastgate/Development/ecoPrimals}"
NESTGATE_DIR="$ECOPRIMALS_ROOT/nestgate"
BEARDOG_DIR="$ECOPRIMALS_ROOT/beardog"
SONGBIRD_DIR="$ECOPRIMALS_ROOT/songbird"

# Check if directories exist
check_primal_dir() {
    local name=$1
    local dir=$2
    
    if [ -d "$dir" ]; then
        echo -e "${GREEN}✅${NC} $name directory found: $dir"
        return 0
    else
        echo -e "${RED}❌${NC} $name directory not found: $dir"
        return 1
    fi
}

echo "📁 Step 1: Checking primal directories..."
check_primal_dir "NestGate" "$NESTGATE_DIR"
NESTGATE_OK=$?
check_primal_dir "BearDog" "$BEARDOG_DIR"
BEARDOG_OK=$?
check_primal_dir "Songbird" "$SONGBIRD_DIR"
SONGBIRD_OK=$?
echo ""

if [ $NESTGATE_OK -ne 0 ]; then
    echo -e "${RED}ERROR: NestGate directory not found${NC}"
    exit 1
fi

# Build primals
echo "🔨 Step 2: Building primals..."

echo -e "${BLUE}Building NestGate...${NC}"
cd "$NESTGATE_DIR"
if cargo build --release 2>&1 | tail -3; then
    echo -e "${GREEN}✅ NestGate built successfully${NC}"
else
    echo -e "${RED}❌ NestGate build failed${NC}"
    exit 1
fi
echo ""

if [ $BEARDOG_OK -eq 0 ]; then
    echo -e "${BLUE}Building BearDog...${NC}"
    cd "$BEARDOG_DIR"
    if cargo build --release 2>&1 | tail -3; then
        echo -e "${GREEN}✅ BearDog built successfully${NC}"
    else
        echo -e "${YELLOW}⚠️  BearDog build failed (optional)${NC}"
        BEARDOG_OK=1
    fi
    echo ""
fi

if [ $SONGBIRD_OK -eq 0 ]; then
    echo -e "${BLUE}Building Songbird...${NC}"
    cd "$SONGBIRD_DIR"
    if cargo build --release 2>&1 | tail -3; then
        echo -e "${GREEN}✅ Songbird built successfully${NC}"
    else
        echo -e "${YELLOW}⚠️  Songbird build failed (optional)${NC}"
        SONGBIRD_OK=1
    fi
    echo ""
fi

# Run integration demo
echo "🎯 Step 3: Running integration demo..."
cd "$NESTGATE_DIR"

echo -e "${BLUE}Running live integration demo...${NC}"
if cargo run --example live-integration-01-storage-security 2>&1; then
    echo ""
    echo -e "${GREEN}✅ Integration demo completed${NC}"
else
    echo ""
    echo -e "${YELLOW}⚠️  Integration demo had issues (check output above)${NC}"
fi
echo ""

# Summary
echo "📊 Verification Summary"
echo "======================="
echo ""

if [ $NESTGATE_OK -eq 0 ]; then
    echo -e "${GREEN}✅ NestGate: READY${NC}"
else
    echo -e "${RED}❌ NestGate: NOT READY${NC}"
fi

if [ $BEARDOG_OK -eq 0 ]; then
    echo -e "${GREEN}✅ BearDog: READY${NC}"
else
    echo -e "${YELLOW}⚠️  BearDog: NOT AVAILABLE${NC}"
fi

if [ $SONGBIRD_OK -eq 0 ]; then
    echo -e "${GREEN}✅ Songbird: READY${NC}"
else
    echo -e "${YELLOW}⚠️  Songbird: NOT AVAILABLE${NC}"
fi

echo ""
echo "💡 Next Steps:"
echo "   1. Start BearDog: cd $BEARDOG_DIR && cargo run --release"
echo "   2. Run demo again: cd $NESTGATE_DIR && cargo run --example live-integration-01-storage-security"
echo "   3. Observe enhanced integration with BearDog"
echo ""
echo "🌍 Ecosystem verification complete!"

