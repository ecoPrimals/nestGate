#!/usr/bin/env bash
# Check prerequisites for NestGate showcase
# Verifies all required tools and dependencies are available

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

ERRORS=0
WARNINGS=0

check_command() {
    local cmd=$1
    local required=$2
    local install_hint=$3
    
    if command -v "$cmd" &> /dev/null; then
        echo -e "${GREEN}✓${NC} $cmd found: $(command -v $cmd)"
        return 0
    else
        if [ "$required" = "required" ]; then
            echo -e "${RED}✗${NC} $cmd not found (REQUIRED)"
            echo -e "  ${YELLOW}Install:${NC} $install_hint"
            ((ERRORS++))
        else
            echo -e "${YELLOW}⚠${NC} $cmd not found (optional)"
            echo -e "  ${YELLOW}Install:${NC} $install_hint"
            ((WARNINGS++))
        fi
        return 1
    fi
}

check_file() {
    local file=$1
    local desc=$2
    
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC} $desc: $file"
        return 0
    else
        echo -e "${RED}✗${NC} $desc not found: $file"
        ((ERRORS++))
        return 1
    fi
}

check_directory() {
    local dir=$1
    local desc=$2
    
    if [ -d "$dir" ]; then
        echo -e "${GREEN}✓${NC} $desc: $dir"
        return 0
    else
        echo -e "${YELLOW}⚠${NC} $desc not found: $dir"
        ((WARNINGS++))
        return 1
    fi
}

check_port() {
    local port=$1
    local desc=$2
    
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠${NC} Port $port ($desc) is in use"
        echo -e "  Process: $(lsof -Pi :$port -sTCP:LISTEN | tail -n 1)"
        ((WARNINGS++))
        return 1
    else
        echo -e "${GREEN}✓${NC} Port $port ($desc) is available"
        return 0
    fi
}

main() {
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║          NestGate Showcase Prerequisites Check            ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    
    echo -e "${BLUE}==>${NC} Checking required commands..."
    check_command "cargo" "required" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    check_command "curl" "required" "apt-get install curl (or yum/brew)"
    check_command "bash" "required" "Should be pre-installed"
    echo ""
    
    echo -e "${BLUE}==>${NC} Checking optional commands..."
    check_command "jq" "optional" "apt-get install jq (or yum/brew)"
    check_command "docker" "optional" "https://docs.docker.com/get-docker/"
    check_command "lsof" "optional" "apt-get install lsof"
    echo ""
    
    echo -e "${BLUE}==>${NC} Checking NestGate build..."
    NESTGATE_ROOT="$(cd ../../.. && pwd)"
    check_file "$NESTGATE_ROOT/Cargo.toml" "NestGate Cargo.toml"
    
    if [ -f "$NESTGATE_ROOT/target/release/nestgate" ]; then
        echo -e "${GREEN}✓${NC} NestGate binary found (release)"
        "$NESTGATE_ROOT/target/release/nestgate" --version 2>/dev/null || echo -e "${YELLOW}  (version check failed)${NC}"
    elif [ -f "$NESTGATE_ROOT/target/debug/nestgate" ]; then
        echo -e "${YELLOW}⚠${NC} NestGate binary found (debug only)"
        echo -e "  ${YELLOW}Recommend building release:${NC} cargo build --release"
    else
        echo -e "${RED}✗${NC} NestGate binary not found"
        echo -e "  ${YELLOW}Build with:${NC} cd $NESTGATE_ROOT && cargo build --release"
        ((ERRORS++))
    fi
    echo ""
    
    echo -e "${BLUE}==>${NC} Checking ports..."
    check_port 8080 "NestGate API"
    check_port 9090 "NestGate Metrics"
    check_port 50051 "NestGate RPC"
    echo ""
    
    echo -e "${BLUE}==>${NC} Checking showcase structure..."
    check_directory "$(pwd)/01_isolated" "Level 1 demos"
    check_directory "$(pwd)/02_ecosystem_integration" "Level 2 demos"
    check_directory "$(pwd)/utils" "Utility scripts"
    echo ""
    
    echo -e "${BLUE}==>${NC} Checking system resources..."
    
    # Check disk space
    AVAILABLE_GB=$(df -BG . | tail -1 | awk '{print $4}' | tr -d 'G')
    if [ "$AVAILABLE_GB" -gt 10 ]; then
        echo -e "${GREEN}✓${NC} Disk space: ${AVAILABLE_GB}GB available"
    else
        echo -e "${YELLOW}⚠${NC} Disk space: ${AVAILABLE_GB}GB available (recommend >10GB)"
        ((WARNINGS++))
    fi
    
    # Check memory
    TOTAL_MEM_GB=$(free -g | awk '/^Mem:/{print $2}')
    if [ "$TOTAL_MEM_GB" -gt 4 ]; then
        echo -e "${GREEN}✓${NC} Memory: ${TOTAL_MEM_GB}GB total"
    else
        echo -e "${YELLOW}⚠${NC} Memory: ${TOTAL_MEM_GB}GB total (recommend >4GB)"
        ((WARNINGS++))
    fi
    
    echo ""
    echo "═══════════════════════════════════════════════════════════"
    
    if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
        echo -e "${GREEN}✓ All checks passed!${NC} You're ready to run the showcase."
        echo ""
        echo "Next steps:"
        echo "  1. Start with: cd 01_isolated/01_storage_basics && ./demo.sh"
        echo "  2. Or read: cat ../00_START_HERE.md"
        return 0
    elif [ $ERRORS -eq 0 ]; then
        echo -e "${YELLOW}⚠ $WARNINGS warning(s)${NC} - You can proceed, but some features may not work."
        echo ""
        echo "You can still run basic demos:"
        echo "  cd 01_isolated/01_storage_basics && ./demo.sh"
        return 0
    else
        echo -e "${RED}✗ $ERRORS error(s)${NC} - Please fix the errors above before proceeding."
        if [ $WARNINGS -gt 0 ]; then
            echo -e "${YELLOW}⚠ $WARNINGS warning(s)${NC}"
        fi
        return 1
    fi
}

main "$@"

