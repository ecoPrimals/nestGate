#!/bin/bash
# Error Handling Audit Script
# Identifies all .expect() and .unwrap() calls in production code

set -e

echo "╔════════════════════════════════════════════════════════╗"
echo "║     NESTGATE ERROR HANDLING AUDIT                      ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Production code (excluding tests)
PROD_GLOB='--glob "!**/tests/**" --glob "!**/*_test.rs" --glob "!**/*_tests.rs"'

echo "=== PRODUCTION CODE ANALYSIS ==="

# Count .expect() calls
expect_count=$(rg "\.expect\(" --type rust code/crates/nestgate-*/src \
    --glob '!**/tests/**' \
    --glob '!**/*_test.rs' \
    --glob '!**/*_tests.rs' \
    2>/dev/null | wc -l)

# Count .unwrap() calls  
unwrap_count=$(rg "\.unwrap\(\)" --type rust code/crates/nestgate-*/src \
    --glob '!**/tests/**' \
    --glob '!**/*_test.rs' \
    --glob '!**/*_tests.rs' \
    2>/dev/null | wc -l)

total=$((expect_count + unwrap_count))

echo "Production .expect() calls:  $expect_count"
echo "Production .unwrap() calls:  $unwrap_count"
echo "Total panic risks:           $total"
echo ""

if [ "$total" -gt 500 ]; then
    echo -e "${RED}❌ Status: HIGH RISK - Immediate migration needed${NC}"
elif [ "$total" -gt 200 ]; then
    echo -e "${YELLOW}⚠️  Status: MODERATE RISK - Migration recommended${NC}"
else
    echo -e "${GREEN}✅ Status: LOW RISK - Minimal migration needed${NC}"
fi

echo ""
echo "=== BY CRATE ==="
for crate in code/crates/nestgate-*/; do
    if [ -d "$crate/src" ]; then
        count=$(rg "\.expect\(|\.unwrap\(\)" --type rust "$crate/src" \
            --glob '!**/tests/**' \
            --glob '!**/*_test.rs' \
            --glob '!**/*_tests.rs' \
            2>/dev/null | wc -l)
        if [ "$count" -gt 0 ]; then
            crate_name=$(basename "$crate")
            printf "  %-30s %4d instances\n" "$crate_name:" "$count"
        fi
    fi
done | sort -k2 -rn

echo ""
echo "=== TOP 10 FILES WITH MOST ISSUES ==="
rg "\.expect\(|\.unwrap\(\)" --type rust code/crates/nestgate-*/src \
    --glob '!**/tests/**' \
    --glob '!**/*_test.rs' \
    --glob '!**/*_tests.rs' \
    -c 2>/dev/null | sort -t: -k2 -rn | head -10 | \
    awk -F: '{
        split($1, path, "/");
        file = path[length(path)];
        printf "  %3d issues: .../%s\n", $2, file
    }'

echo ""
echo "=== CRITICAL AREAS (High Priority) ==="

# API handlers
api_count=$(rg "\.expect\(|\.unwrap\(\)" --type rust \
    code/crates/nestgate-api/src/handlers \
    --glob '!**/tests/**' \
    2>/dev/null | wc -l)
echo "  API Handlers:        $api_count instances"

# Network operations
network_count=$(rg "\.expect\(|\.unwrap\(\)" --type rust \
    code/crates/nestgate-core/src/network \
    --glob '!**/tests/**' \
    2>/dev/null | wc -l)
echo "  Network Operations:  $network_count instances"

# Config loading
config_count=$(rg "\.expect\(|\.unwrap\(\)" --type rust \
    code/crates/nestgate-core/src/config \
    --glob '!**/tests/**' \
    2>/dev/null | wc -l)
echo "  Config Loading:      $config_count instances"

echo ""
echo "=== TEST CODE ANALYSIS ==="
test_count=$(rg "\.expect\(|\.unwrap\(\)" --type rust \
    code/crates/nestgate-*/src \
    --glob '**/tests/**' \
    2>/dev/null | wc -l)
echo "Test .expect()/.unwrap():    $test_count instances"
echo -e "${GREEN}✅ Test unwraps are acceptable${NC}"

echo ""
echo "=== RECOMMENDATIONS ==="
if [ "$total" -gt 500 ]; then
    echo "1. Start with API handlers (user-facing)"
    echo "2. Focus on network operations (stability)"
    echo "3. Fix config loading (startup reliability)"
    echo "4. Estimate: 12-16 days for full migration"
elif [ "$total" -gt 200 ]; then
    echo "1. Prioritize critical paths"
    echo "2. Migrate high-traffic code first"
    echo "3. Estimate: 8-12 days for migration"
else
    echo "1. Current state is acceptable"
    echo "2. Fix remaining issues opportunistically"
    echo "3. Estimate: 4-6 days for cleanup"
fi

echo ""
echo "Next: Review ERROR_HANDLING_MIGRATION_PLAN.md for migration strategy"

