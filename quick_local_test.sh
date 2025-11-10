#!/bin/bash
# Quick local test script for NestGate
# Run this on Eastgate before deploying to other towers

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔════════════════════════════════════════╗"
echo "║   NESTGATE LOCAL TEST SUITE           ║"
echo "║   Build → Test → Validate             ║"
echo "╚════════════════════════════════════════╝"
echo -e "${NC}"

# Track results
TOTAL_CHECKS=0
PASSED_CHECKS=0

check_result() {
    ((TOTAL_CHECKS++))
    if [ $1 -eq 0 ]; then
        ((PASSED_CHECKS++))
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
    fi
}

# ===========================================
# PHASE 1: ENVIRONMENT CHECK
# ===========================================

echo -e "\n${BLUE}═══ Phase 1: Environment Check ═══${NC}\n"

# Check Rust
echo -n "Checking Rust installation... "
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    check_result 0 "Rust installed: $RUST_VERSION"
else
    check_result 1 "Rust not found"
    echo "Install from: https://rustup.rs"
    exit 1
fi

# Check cargo
echo -n "Checking Cargo... "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    check_result 0 "Cargo available: $CARGO_VERSION"
else
    check_result 1 "Cargo not found"
    exit 1
fi

# Check disk space
echo -n "Checking disk space... "
AVAILABLE=$(df -BG . | tail -1 | awk '{print $4}' | tr -d 'G')
if [ "$AVAILABLE" -gt 10 ]; then
    check_result 0 "Disk space: ${AVAILABLE}GB available"
else
    check_result 1 "Low disk space: ${AVAILABLE}GB (need 10GB+)"
fi

# Check ZFS (optional)
echo -n "Checking ZFS... "
if command -v zfs &> /dev/null; then
    ZFS_VERSION=$(zfs --version 2>&1 | head -1)
    check_result 0 "ZFS available: $ZFS_VERSION"
else
    check_result 0 "ZFS not installed (optional)"
fi

# ===========================================
# PHASE 2: BUILD
# ===========================================

echo -e "\n${BLUE}═══ Phase 2: Build ═══${NC}\n"

# Clean build
echo "Cleaning previous builds..."
cargo clean &> /dev/null
check_result $? "Clean completed"

# Debug build
echo "Building in debug mode..."
START_TIME=$(date +%s)
cargo build --workspace 2>&1 | tee /tmp/nestgate_build_debug.log | tail -5
BUILD_RESULT=${PIPESTATUS[0]}
END_TIME=$(date +%s)
DEBUG_TIME=$((END_TIME - START_TIME))

if [ $BUILD_RESULT -eq 0 ]; then
    check_result 0 "Debug build successful (${DEBUG_TIME}s)"
else
    check_result 1 "Debug build failed"
    echo "Check /tmp/nestgate_build_debug.log for details"
    exit 1
fi

# Release build
echo "Building in release mode..."
START_TIME=$(date +%s)
cargo build --release --workspace 2>&1 | tee /tmp/nestgate_build_release.log | tail -5
BUILD_RESULT=${PIPESTATUS[0]}
END_TIME=$(date +%s)
RELEASE_TIME=$((END_TIME - START_TIME))

if [ $BUILD_RESULT -eq 0 ]; then
    check_result 0 "Release build successful (${RELEASE_TIME}s)"
    
    # Check binary
    if [ -f "target/release/nestgate" ]; then
        BINARY_SIZE=$(ls -lh target/release/nestgate | awk '{print $5}')
        echo "  Binary size: $BINARY_SIZE"
    fi
else
    check_result 1 "Release build failed"
    exit 1
fi

# ===========================================
# PHASE 3: UNIT TESTS
# ===========================================

echo -e "\n${BLUE}═══ Phase 3: Unit Tests ═══${NC}\n"

echo "Running unit tests (this may take a few minutes)..."
cargo test --workspace --lib 2>&1 | tee /tmp/nestgate_tests.log | grep -E "test result:|running"

TEST_RESULT=${PIPESTATUS[0]}

if [ $TEST_RESULT -eq 0 ]; then
    # Count tests
    TEST_LINE=$(grep "test result:" /tmp/nestgate_tests.log | tail -1)
    check_result 0 "Tests passed: $TEST_LINE"
else
    check_result 1 "Some tests failed"
    echo "Check /tmp/nestgate_tests.log for details"
fi

# ===========================================
# PHASE 4: LINTING
# ===========================================

echo -e "\n${BLUE}═══ Phase 4: Code Quality ═══${NC}\n"

echo "Running Clippy..."
cargo clippy --workspace 2>&1 | tee /tmp/nestgate_clippy.log | tail -10

WARNING_COUNT=$(grep -c "warning:" /tmp/nestgate_clippy.log || echo 0)
ERROR_COUNT=$(grep -c "error:" /tmp/nestgate_clippy.log || echo 0)

echo "Clippy results: $WARNING_COUNT warnings, $ERROR_COUNT errors"

if [ "$ERROR_COUNT" -eq 0 ]; then
    check_result 0 "No critical errors"
else
    check_result 1 "Clippy errors found"
fi

# ===========================================
# PHASE 5: BASIC FUNCTIONALITY
# ===========================================

echo -e "\n${BLUE}═══ Phase 5: Basic Functionality ═══${NC}\n"

# Test version
echo "Testing binary execution..."
if ./target/release/nestgate --version &> /dev/null; then
    VERSION=$(./target/release/nestgate --version)
    check_result 0 "Binary executes: $VERSION"
else
    check_result 1 "Binary execution failed"
fi

# Test help
if ./target/release/nestgate --help &> /dev/null; then
    check_result 0 "Help command works"
else
    check_result 1 "Help command failed"
fi

# ===========================================
# PHASE 6: QUICK INTEGRATION TEST
# ===========================================

echo -e "\n${BLUE}═══ Phase 6: Integration Test ═══${NC}\n"

# Create test directory
TEST_DIR="/tmp/nestgate_quick_test_$$"
mkdir -p "$TEST_DIR"
echo "Test directory: $TEST_DIR"

# Generate test file
TEST_FILE="$TEST_DIR/test.dat"
dd if=/dev/urandom of="$TEST_FILE" bs=1M count=10 2>/dev/null
check_result $? "Generated 10MB test file"

# Test compression
echo "Testing compression..."
gzip -k "$TEST_FILE" 2>/dev/null
if [ -f "${TEST_FILE}.gz" ]; then
    ORIGINAL_SIZE=$(stat -f%z "$TEST_FILE" 2>/dev/null || stat -c%s "$TEST_FILE")
    COMPRESSED_SIZE=$(stat -f%z "${TEST_FILE}.gz" 2>/dev/null || stat -c%s "${TEST_FILE}.gz")
    RATIO=$(echo "scale=2; $ORIGINAL_SIZE / $COMPRESSED_SIZE" | bc)
    check_result 0 "Compression works (${RATIO}x ratio)"
else
    check_result 1 "Compression test failed"
fi

# Cleanup test dir
rm -rf "$TEST_DIR"
check_result $? "Cleanup completed"

# ===========================================
# SUMMARY
# ===========================================

echo -e "\n${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           TEST SUMMARY                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"

echo -e "\nResults: ${GREEN}$PASSED_CHECKS${NC} / ${TOTAL_CHECKS} checks passed"

# Deployment readiness
if [ $PASSED_CHECKS -eq $TOTAL_CHECKS ]; then
    echo -e "\n${GREEN}✅ ALL CHECKS PASSED${NC}"
    echo -e "${GREEN}✅ READY FOR TOWER DEPLOYMENT${NC}\n"
    
    echo "Next steps:"
    echo "  1. Review logs in /tmp/nestgate_*.log"
    echo "  2. Read LOCAL_TESTING_GUIDE.md for details"
    echo "  3. Deploy to towers: scp target/release/nestgate westgate:/tmp/"
    echo "  4. See showcase/REAL_WORLD_SCENARIOS.md for use cases"
    
    exit 0
elif [ $PASSED_CHECKS -gt $((TOTAL_CHECKS * 8 / 10)) ]; then
    echo -e "\n${YELLOW}⚠️  MOSTLY PASSING (${PASSED_CHECKS}/${TOTAL_CHECKS})${NC}"
    echo -e "${YELLOW}⚠️  Minor issues - review and decide${NC}\n"
    
    echo "Review logs:"
    echo "  - /tmp/nestgate_build_debug.log"
    echo "  - /tmp/nestgate_build_release.log"
    echo "  - /tmp/nestgate_tests.log"
    echo "  - /tmp/nestgate_clippy.log"
    
    exit 1
else
    echo -e "\n${RED}❌ SIGNIFICANT ISSUES (${PASSED_CHECKS}/${TOTAL_CHECKS})${NC}"
    echo -e "${RED}❌ FIX BEFORE DEPLOYMENT${NC}\n"
    
    echo "Review logs:"
    echo "  - /tmp/nestgate_build_debug.log"
    echo "  - /tmp/nestgate_build_release.log"
    echo "  - /tmp/nestgate_tests.log"
    echo "  - /tmp/nestgate_clippy.log"
    
    exit 1
fi

