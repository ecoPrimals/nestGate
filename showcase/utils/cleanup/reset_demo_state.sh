#!/usr/bin/env bash
# Reset all demo state - clean up data from showcase demos

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

NESTGATE_URL="${NESTGATE_URL:-http://localhost:8080}"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                                                            ║"
echo "║          NestGate Showcase - Reset Demo State             ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

echo -e "${YELLOW}Warning:${NC} This will delete all demo datasets and data."
echo ""
read -p "Are you sure you want to continue? [y/N] " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cancelled."
    exit 0
fi

echo ""
echo "Cleaning up demo datasets..."
echo ""

# List of known demo datasets
DEMO_DATASETS=(
    "demo-storage-basics"
    "demo-data-services"
    "demo-capability-discovery"
    "demo-health-monitoring"
    "demo-zfs-advanced"
    "users"
    "photos"
    "logs"
    "docs"
    "stress"
    "large"
    "search"
)

CLEANED=0
ERRORS=0

for dataset in "${DEMO_DATASETS[@]}"; do
    echo -n "Deleting $dataset... "
    if curl -s -f -X DELETE "${NESTGATE_URL}/api/v1/datasets/$dataset" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        ((CLEANED++))
    else
        echo -e "${YELLOW}(not found or error)${NC}"
        ((ERRORS++))
    fi
done

echo ""
echo "═══════════════════════════════════════════════════════════"

if [ $CLEANED -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Cleaned up $CLEANED dataset(s)"
fi

if [ $ERRORS -gt 0 ]; then
    echo -e "${YELLOW}⚠${NC} $ERRORS dataset(s) not found or could not be deleted"
fi

echo ""
echo "Demo state has been reset."
echo "You can now run demos with a clean slate."
echo ""

