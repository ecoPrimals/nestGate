#!/usr/bin/env bash
#
# Comprehensive Test Runner for NestGate
#
# Runs all test suites: unit, integration, E2E, chaos, concurrency

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}        🧪 NESTGATE COMPREHENSIVE TEST SUITE 🧪${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════${NC}"
echo ""

# Track results
PASSED=0
FAILED=0
TOTAL=0

run_test_suite() {
    local name="$1"
    local command="$2"
    
    echo -e "${YELLOW}━━━ Running: $name ━━━${NC}"
    TOTAL=$((TOTAL + 1))
    
    if eval "$command"; then
        echo -e "${GREEN}✅ PASSED: $name${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}❌ FAILED: $name${NC}"
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

# 1. Unit Tests - UniBin CLI Parsing
run_test_suite "UniBin CLI Parsing (Unit)" \
    "cargo test --test unibin_unit_cli_parsing -- --nocapture"

# 2. E2E Tests - UniBin Daemon Lifecycle
run_test_suite "UniBin Daemon Lifecycle (E2E)" \
    "cargo test --test unibin_e2e_daemon_lifecycle -- --nocapture --test-threads=1"

# 3. Concurrency Tests - DashMap WebSocket
run_test_suite "DashMap WebSocket Concurrency" \
    "cargo test --test dashmap_concurrent_websocket -- --nocapture"

# 4. Chaos Tests - RPC Fault Injection
run_test_suite "RPC Fault Injection (Chaos)" \
    "cargo test --test chaos_fault_injection_rpc -- --nocapture"

# 5. Integration Tests - Unix Socket Lifecycle
run_test_suite "Unix Socket RPC Lifecycle (Integration)" \
    "cargo test --test integration_unix_socket_lifecycle -- --nocapture"

# 6. Existing Concurrent Tests
run_test_suite "Existing Concurrent Operations" \
    "cargo test concurrent_operations -- --nocapture"

# 7. Existing Chaos Tests
run_test_suite "Existing Chaos Engineering Suite" \
    "cargo test chaos_engineering -- --nocapture"

# 8. All Unit Tests (other)
run_test_suite "All Other Unit Tests" \
    "cargo test --lib -- --nocapture"

# 9. Doc Tests
run_test_suite "Documentation Tests" \
    "cargo test --doc -- --nocapture"

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}                    📊 TEST RESULTS 📊${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "Total Suites:  $TOTAL"
echo -e "${GREEN}Passed:        $PASSED${NC}"
echo -e "${RED}Failed:        $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}        ✨ ALL TESTS PASSED! ✨${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    exit 0
else
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${RED}        ⚠️  SOME TESTS FAILED ⚠️${NC}"
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    exit 1
fi
