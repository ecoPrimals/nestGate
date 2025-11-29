#!/bin/bash
# Hardcoding Audit Script
# Identifies all hardcoded values in production code

set -e

echo "╔════════════════════════════════════════════════════════╗"
echo "║     NESTGATE HARDCODING AUDIT                          ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Count functions
count_pattern() {
    local pattern=$1
    local description=$2
    local count=$(rg "$pattern" --type rust code/crates/nestgate-*/src \
        --glob '!**/tests/**' \
        --glob '!**/*_test.rs' \
        --glob '!**/*_tests.rs' \
        2>/dev/null | wc -l)
    
    if [ "$count" -gt 100 ]; then
        echo -e "${RED}❌ $description: $count instances${NC}"
    elif [ "$count" -gt 50 ]; then
        echo -e "${YELLOW}⚠️  $description: $count instances${NC}"
    else
        echo -e "${GREEN}✅ $description: $count instances${NC}"
    fi
}

echo "=== PORT REFERENCES ==="
count_pattern "\b8080\b" "Port 8080 (HTTP API)"
count_pattern "\b3000\b" "Port 3000 (Dev server)"
count_pattern "\b5432\b" "Port 5432 (PostgreSQL)"
count_pattern "\b6379\b" "Port 6379 (Redis)"
count_pattern "\b9090\b" "Port 9090 (Prometheus)"
count_pattern "\b27017\b" "Port 27017 (MongoDB)"

echo ""
echo "=== IP/HOSTNAME REFERENCES ==="
count_pattern "127\.0\.0\.1" "Localhost IP (127.0.0.1)"
count_pattern "\blocalhost\b" "Localhost hostname"

echo ""
echo "=== PATH REFERENCES ==="
count_pattern "/opt/nestgate" "Hardcoded path: /opt/nestgate"
count_pattern "/etc/nestgate" "Hardcoded path: /etc/nestgate"

echo ""
echo "=== TOP FILES BY HARDCODED VALUES ==="
rg "\b(8080|3000|5432|6379|9090|27017|127\.0\.0\.1|localhost)\b" \
    --type rust code/crates/nestgate-*/src \
    --glob '!**/tests/**' \
    --glob '!**/*_test.rs' \
    --glob '!**/*_tests.rs' \
    -c 2>/dev/null | sort -t: -k2 -rn | head -10 | \
    awk -F: '{printf "  %3d instances: %s\n", $2, $1}'

echo ""
echo "=== SUMMARY ==="
total=$(rg "\b(8080|3000|5432|6379|9090|27017|127\.0\.0\.1|localhost)\b" \
    --type rust code/crates/nestgate-*/src \
    --glob '!**/tests/**' \
    --glob '!**/*_test.rs' \
    --glob '!**/*_tests.rs' \
    2>/dev/null | wc -l)

if [ "$total" -gt 500 ]; then
    echo -e "${RED}❌ Total hardcoded values: $total (NEEDS ATTENTION)${NC}"
elif [ "$total" -gt 200 ]; then
    echo -e "${YELLOW}⚠️  Total hardcoded values: $total (SHOULD MIGRATE)${NC}"
else
    echo -e "${GREEN}✅ Total hardcoded values: $total (ACCEPTABLE)${NC}"
fi

echo ""
echo "Next: Review HARDCODING_MIGRATION_PLAN.md for migration strategy"

