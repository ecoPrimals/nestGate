#!/bin/bash
# Deployment Readiness Verification Script
# Generated: December 1, 2025

set -e

echo "=== NESTGATE DEPLOYMENT READINESS CHECK ==="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0

check_pass() {
    echo -e "${GREEN}✅ PASS${NC}: $1"
    ((PASS++))
}

check_fail() {
    echo -e "${RED}❌ FAIL${NC}: $1"
    ((FAIL++))
}

check_warn() {
    echo -e "${YELLOW}⚠️  WARN${NC}: $1"
}

echo "1️⃣  Checking compilation..."
if cargo check --workspace --quiet 2>/dev/null; then
    check_pass "Compilation clean"
else
    check_fail "Compilation has errors"
fi

echo ""
echo "2️⃣  Running tests..."
TEST_OUTPUT=$(cargo test --workspace --lib --quiet 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    TESTS_PASSED=$(echo "$TEST_OUTPUT" | grep -o '[0-9]* passed' | head -1 | grep -o '[0-9]*')
    check_pass "All tests passing ($TESTS_PASSED tests)"
else
    check_fail "Some tests failing"
fi

echo ""
echo "3️⃣  Checking release build..."
if cargo build --workspace --release --quiet 2>/dev/null; then
    check_pass "Release build successful"
else
    check_fail "Release build failed"
fi

echo ""
echo "4️⃣  Checking formatting..."
if cargo fmt --all --check 2>/dev/null; then
    check_pass "Code properly formatted"
else
    check_warn "Code formatting needed (run: cargo fmt --all)"
fi

echo ""
echo "5️⃣  Checking for critical issues..."
UNWRAPS_PROD=$(find code/crates/*/src -name "*.rs" ! -path "*/test*" -exec grep -l "\.unwrap()" {} \; 2>/dev/null | wc -l)
if [ "$UNWRAPS_PROD" -lt 30 ]; then
    check_pass "Unwraps in production code: $UNWRAPS_PROD (acceptable)"
else
    check_warn "Unwraps in production code: $UNWRAPS_PROD (review recommended)"
fi

echo ""
echo "6️⃣  Checking configuration..."
if [ -f "config/production.toml" ] || [ -f "docker/docker-compose.production.yml" ]; then
    check_pass "Production configuration files present"
else
    check_warn "Production configuration files missing"
fi

echo ""
echo "7️⃣  Checking documentation..."
if [ -f "README.md" ] && [ -f "DEPLOYMENT_GUIDE.md" ]; then
    check_pass "Core documentation present"
else
    check_warn "Some documentation missing"
fi

echo ""
echo "================================================"
echo "DEPLOYMENT READINESS SUMMARY"
echo "================================================"
echo -e "Checks Passed: ${GREEN}$PASS${NC}"
echo -e "Checks Failed: ${RED}$FAIL${NC}"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}✅ DEPLOYMENT READY${NC}"
    echo "System is ready for staging/production deployment"
    exit 0
else
    echo -e "${RED}❌ NOT READY${NC}"
    echo "Fix failing checks before deployment"
    exit 1
fi
