#!/usr/bin/env bash
# Run all Level 1 (Isolated) demos sequentially

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

TOTAL_DEMOS=5
COMPLETED=0
FAILED=0

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                                                            ║"
echo "║      NestGate Showcase - Level 1: Isolated Instance       ║"
echo "║                                                            ║"
echo "║  Running all 5 demos sequentially...                      ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

run_demo() {
    local demo_dir=$1
    local demo_name=$2
    local demo_num=$3
    
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Demo $demo_num/$TOTAL_DEMOS: $demo_name${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    if [ -f "$demo_dir/demo.sh" ]; then
        if (cd "$demo_dir" && ./demo.sh); then
            ((COMPLETED++))
            echo -e "${GREEN}✓ Demo $demo_num complete${NC}"
        else
            ((FAILED++))
            echo -e "${YELLOW}⚠ Demo $demo_num failed or skipped${NC}"
        fi
    else
        echo -e "${YELLOW}⚠ Demo script not found: $demo_dir/demo.sh${NC}"
        echo -e "${YELLOW}  (Demo may not be implemented yet)${NC}"
        ((FAILED++))
    fi
    
    # Brief pause between demos
    if [ $demo_num -lt $TOTAL_DEMOS ]; then
        echo ""
        echo "Press Enter to continue to next demo (or Ctrl+C to stop)..."
        read -r
    fi
}

# Check if NestGate is running
echo "Checking prerequisites..."
if ! curl -s -f http://localhost:8080/health > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠ NestGate doesn't appear to be running${NC}"
    echo ""
    read -p "Would you like to start it now? [y/N] " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if [ -f "../../scripts/start_data_service.sh" ]; then
            ../../scripts/start_data_service.sh
            sleep 3
        else
            echo "Start script not found. Please start NestGate manually."
            exit 1
        fi
    else
        echo "Please start NestGate before running demos:"
        echo "  ../../scripts/start_data_service.sh"
        exit 1
    fi
fi

echo -e "${GREEN}✓ NestGate is running${NC}"
echo ""

# Run all demos
run_demo "01_storage_basics" "Storage Basics" 1
run_demo "02_data_services" "Data Services API" 2
run_demo "03_capability_discovery" "Capability Discovery" 3
run_demo "04_health_monitoring" "Health Monitoring" 4
run_demo "05_zfs_advanced" "ZFS Advanced Features" 5

# Summary
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                                                            ║"
echo "║                 Level 1 Complete!                         ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Results:"
echo -e "  ${GREEN}✓ Completed: $COMPLETED/$TOTAL_DEMOS${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "  ${YELLOW}⚠ Failed/Skipped: $FAILED/$TOTAL_DEMOS${NC}"
fi
echo ""

if [ $COMPLETED -eq $TOTAL_DEMOS ]; then
    echo -e "${GREEN}🎉 Congratulations! You've completed all Level 1 demos!${NC}"
    echo ""
    echo "What you've learned:"
    echo "  ✓ Basic storage operations"
    echo "  ✓ REST API usage"
    echo "  ✓ Zero-knowledge architecture"
    echo "  ✓ Health monitoring"
    echo "  ✓ Advanced ZFS features"
    echo ""
    echo "Ready for Level 2: Ecosystem Integration"
    echo "  cd ../02_ecosystem_integration"
    echo "  cat README.md"
else
    echo "Some demos were not completed."
    echo "Review any errors above and try again."
fi

echo ""

