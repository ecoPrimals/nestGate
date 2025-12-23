#!/bin/bash
# 🎯 VERIFICATION - Test All Local Showcase Demos
# Quick smoke test to verify all demos work

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "🎯 NestGate Local Showcase - Smoke Test"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

PASS=0
FAIL=0
TESTS=()

run_test() {
    local name="$1"
    local cmd="$2"
    
    echo -e "${BLUE}Testing: $name${NC}"
    
    if timeout 30 bash -c "$cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}\n"
        PASS=$((PASS + 1))
        TESTS+=("✅ $name")
    else
        echo -e "${RED}❌ FAIL${NC}\n"
        FAIL=$((FAIL + 1))
        TESTS+=("❌ $name")
    fi
}

# Test 1: Hello World Demo
run_test "01-hello-storage (Hello World)" \
    "cd 01-hello-storage && ./demo-hello-world.sh"

# Test 2: Snapshots Demo
run_test "02-zfs-magic (Snapshots)" \
    "cd 02-zfs-magic && ./demo-snapshots.sh"

# Test 3: Compression Demo
run_test "02-zfs-magic (Compression)" \
    "cd 02-zfs-magic && ./demo-compression.sh"

# Test 4: Discovery Demo
run_test "04-self-awareness (Discovery)" \
    "cd 04-self-awareness && ./demo-discovery.sh"

# Test 5: Fallback Demo
run_test "04-self-awareness (Fallback)" \
    "cd 04-self-awareness && ./demo-fallback.sh"

# Test 6: Throughput Demo
run_test "05-performance (Throughput)" \
    "cd 05-performance && ./demo-throughput.sh"

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Test Results:"
for test in "${TESTS[@]}"; do
    echo "  $test"
done
echo ""
echo "Summary:"
echo -e "  ${GREEN}Passed: $PASS${NC}"
if [ $FAIL -gt 0 ]; then
    echo -e "  ${RED}Failed: $FAIL${NC}"
fi
echo ""

TOTAL=$((PASS + FAIL))
PASS_RATE=$((PASS * 100 / TOTAL))

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed! (${PASS_RATE}%)${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed (${PASS_RATE}% pass rate)${NC}"
    exit 1
fi

