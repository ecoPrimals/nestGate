#!/usr/bin/env bash
# Comprehensive test suite for all NestGate showcase demos
# Tests demos with live services and generates detailed report

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NESTGATE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
NESTGATE_URL="${NESTGATE_URL:-http://localhost:8080}"
TEST_MODE="${TEST_MODE:-interactive}"  # interactive or automated
LOG_DIR="$SCRIPT_DIR/test_logs"
REPORT_FILE="$SCRIPT_DIR/TEST_REPORT_$(date +%Y%m%d_%H%M%S).md"

# Stats
TESTS_TOTAL=0
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0
START_TIME=$(date +%s)

# Ensure log directory exists
mkdir -p "$LOG_DIR"

# =============================================================================
# Helper Functions
# =============================================================================

log_header() {
    echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║$(printf '%*s' 60 '' | tr ' ' ' ')║${NC}"
    echo -e "${BLUE}║  ${1}$(printf '%*s' $((57 - ${#1})) '' | tr ' ' ' ')║${NC}"
    echo -e "${BLUE}║$(printf '%*s' 60 '' | tr ' ' ' ')║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}\n"
}

log_section() {
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}${1}${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"
}

log_test() {
    echo -e "${BLUE}→${NC} Testing: ${1}"
}

log_pass() {
    echo -e "${GREEN}✓${NC} ${1}"
    ((TESTS_PASSED++))
}

log_fail() {
    echo -e "${RED}✗${NC} ${1}"
    ((TESTS_FAILED++))
}

log_skip() {
    echo -e "${YELLOW}⊘${NC} ${1}"
    ((TESTS_SKIPPED++))
}

log_info() {
    echo -e "${YELLOW}ℹ${NC} ${1}"
}

log_error() {
    echo -e "${RED}ERROR:${NC} ${1}" >&2
}

# =============================================================================
# Service Management
# =============================================================================

check_nestgate_service() {
    log_section "Checking NestGate Service"
    
    if curl -sf "${NESTGATE_URL}/health" > /dev/null 2>&1; then
        log_pass "NestGate service is running at ${NESTGATE_URL}"
        return 0
    else
        log_fail "NestGate service is NOT running at ${NESTGATE_URL}"
        log_info "Start with: cd $NESTGATE_ROOT && ./start_local_dev.sh"
        return 1
    fi
}

check_prerequisites() {
    log_section "Checking Prerequisites"
    
    local missing=0
    
    # Required commands
    for cmd in curl bash jq; do
        if command -v "$cmd" &> /dev/null; then
            log_pass "$cmd found"
        else
            log_fail "$cmd not found (required)"
            ((missing++))
        fi
    done
    
    # Optional commands
    for cmd in docker lsof; do
        if command -v "$cmd" &> /dev/null; then
            log_pass "$cmd found (optional)"
        else
            log_skip "$cmd not found (optional)"
        fi
    done
    
    return $missing
}

# =============================================================================
# Demo Testing Functions
# =============================================================================

test_demo() {
    local demo_path=$1
    local demo_name=$2
    local level=$3
    local required_service=${4:-nestgate}
    
    ((TESTS_TOTAL++))
    
    log_test "Level $level: $demo_name"
    
    local log_file="$LOG_DIR/${level}_${demo_name//[^a-zA-Z0-9]/_}.log"
    local start=$(date +%s)
    
    # Check if demo script exists
    if [ ! -f "$demo_path/demo.sh" ]; then
        log_skip "$demo_name (demo.sh not found)"
        echo "SKIP: demo.sh not found" > "$log_file"
        return 1
    fi
    
    # Check if required service is available
    if [ "$required_service" != "nestgate" ]; then
        log_skip "$demo_name (requires $required_service - not available)"
        echo "SKIP: Requires $required_service" > "$log_file"
        return 1
    fi
    
    # Run the demo in test mode
    cd "$demo_path"
    
    if [ "$TEST_MODE" = "automated" ]; then
        # Automated mode: non-interactive with timeout
        if timeout 60s bash -c "yes n | ./demo.sh" > "$log_file" 2>&1; then
            local duration=$(($(date +%s) - start))
            log_pass "$demo_name (${duration}s)"
            echo "PASS: Completed in ${duration}s" >> "$log_file"
            return 0
        else
            local duration=$(($(date +%s) - start))
            log_fail "$demo_name (${duration}s) - see $log_file"
            echo "FAIL: Error or timeout after ${duration}s" >> "$log_file"
            return 1
        fi
    else
        # Interactive mode: manual confirmation
        if ./demo.sh > "$log_file" 2>&1; then
            local duration=$(($(date +%s) - start))
            log_pass "$demo_name (${duration}s)"
            echo "PASS: Completed in ${duration}s" >> "$log_file"
            return 0
        else
            local duration=$(($(date +%s) - start))
            log_fail "$demo_name (${duration}s) - see $log_file"
            echo "FAIL: Error after ${duration}s" >> "$log_file"
            return 1
        fi
    fi
}

# =============================================================================
# Test Levels
# =============================================================================

test_level_1() {
    log_section "Testing Level 1: Isolated Instance (5 demos)"
    
    local level1_dir="$SCRIPT_DIR/01_isolated"
    
    test_demo "$level1_dir/01_storage_basics" "Storage Basics" 1 "nestgate" || true
    test_demo "$level1_dir/02_data_services" "Data Services" 1 "nestgate" || true
    test_demo "$level1_dir/03_capability_discovery" "Capability Discovery" 1 "nestgate" || true
    test_demo "$level1_dir/04_health_monitoring" "Health Monitoring" 1 "nestgate" || true
    test_demo "$level1_dir/05_zfs_advanced" "ZFS Advanced" 1 "nestgate" || true
}

test_level_2() {
    log_section "Testing Level 2: Ecosystem Integration (4 demos)"
    
    local level2_dir="$SCRIPT_DIR/02_ecosystem_integration"
    
    # Check for external services
    local has_beardog=false
    local has_songbird=false
    local has_toadstool=false
    
    curl -sf http://localhost:8000/health > /dev/null 2>&1 && has_beardog=true
    curl -sf http://localhost:9090/health > /dev/null 2>&1 && has_songbird=true
    curl -sf http://localhost:7000/health > /dev/null 2>&1 && has_toadstool=true
    
    # Test demos (will skip if services unavailable)
    if $has_beardog; then
        test_demo "$level2_dir/01_beardog_crypto" "BearDog Crypto" 2 "nestgate" || true
    else
        ((TESTS_TOTAL++))
        log_skip "BearDog Crypto (BearDog service not running)"
        ((TESTS_SKIPPED++))
    fi
    
    if $has_songbird; then
        test_demo "$level2_dir/02_songbird_data_service" "Songbird Orchestration" 2 "nestgate" || true
    else
        ((TESTS_TOTAL++))
        log_skip "Songbird Orchestration (Songbird service not running)"
        ((TESTS_SKIPPED++))
    fi
    
    if $has_toadstool; then
        test_demo "$level2_dir/03_toadstool_storage" "ToadStool Storage" 2 "nestgate" || true
    else
        ((TESTS_TOTAL++))
        log_skip "ToadStool Storage (ToadStool service not running)"
        ((TESTS_SKIPPED++))
    fi
    
    # Data flow patterns can run with just NestGate
    test_demo "$level2_dir/04_data_flow_patterns" "Data Flow Patterns" 2 "nestgate" || true
}

test_level_3() {
    log_section "Testing Level 3: Federation (1 demo)"
    
    local level3_dir="$SCRIPT_DIR/03_federation"
    
    # Mesh formation can run in single-node mode
    test_demo "$level3_dir/01_mesh_formation" "Mesh Formation" 3 "nestgate" || true
}

# =============================================================================
# Report Generation
# =============================================================================

generate_report() {
    log_section "Generating Test Report"
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - START_TIME))
    local pass_rate=0
    
    if [ $TESTS_TOTAL -gt 0 ]; then
        pass_rate=$((TESTS_PASSED * 100 / TESTS_TOTAL))
    fi
    
    cat > "$REPORT_FILE" <<EOF
# NestGate Showcase Test Report

**Date**: $(date '+%Y-%m-%d %H:%M:%S')  
**Duration**: ${total_duration}s  
**Mode**: $TEST_MODE

---

## 📊 Summary

\`\`\`
Total Tests:    $TESTS_TOTAL
Passed:         $TESTS_PASSED ✓
Failed:         $TESTS_FAILED ✗
Skipped:        $TESTS_SKIPPED ⊘
Pass Rate:      ${pass_rate}%
\`\`\`

---

## 🎯 Results by Level

### Level 1: Isolated Instance
- Storage Basics: $(grep "1_Storage_Basics" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "✗ FAIL/SKIP")
- Data Services: $(grep "1_Data_Services" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "✗ FAIL/SKIP")
- Capability Discovery: $(grep "1_Capability_Discovery" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "✗ FAIL/SKIP")
- Health Monitoring: $(grep "1_Health_Monitoring" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "✗ FAIL/SKIP")
- ZFS Advanced: $(grep "1_ZFS_Advanced" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "✗ FAIL/SKIP")

### Level 2: Ecosystem Integration
- BearDog Crypto: $(grep "2_BearDog_Crypto" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "⊘ SKIP (service required)")
- Songbird Orchestration: $(grep "2_Songbird" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "⊘ SKIP (service required)")
- ToadStool Storage: $(grep "2_ToadStool" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "⊘ SKIP (service required)")
- Data Flow Patterns: $(grep "2_Data_Flow_Patterns" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "✗ FAIL/SKIP")

### Level 3: Federation
- Mesh Formation: $(grep "3_Mesh_Formation" "$LOG_DIR"/*.log 2>/dev/null | head -1 | grep -q PASS && echo "✓ PASS" || echo "✗ FAIL/SKIP")

---

## 📝 Test Logs

All test logs saved to: \`$LOG_DIR/\`

View individual logs:
\`\`\`bash
ls -lh $LOG_DIR/
cat $LOG_DIR/<test_name>.log
\`\`\`

---

## 🔍 Issues Found

EOF

    # Analyze logs for common errors
    if [ $TESTS_FAILED -gt 0 ]; then
        echo "### Failed Tests" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        for log in "$LOG_DIR"/*.log; do
            if grep -q "FAIL" "$log" 2>/dev/null; then
                echo "- $(basename "$log" .log)" >> "$REPORT_FILE"
                echo '  ```' >> "$REPORT_FILE"
                tail -10 "$log" >> "$REPORT_FILE"
                echo '  ```' >> "$REPORT_FILE"
                echo "" >> "$REPORT_FILE"
            fi
        done
    else
        echo "No issues found! All tests passed or were skipped as expected." >> "$REPORT_FILE"
    fi
    
    cat >> "$REPORT_FILE" <<EOF

---

## ✅ Recommendations

EOF

    if [ $pass_rate -eq 100 ]; then
        echo "🎉 Excellent! All tests passed." >> "$REPORT_FILE"
    elif [ $pass_rate -ge 70 ]; then
        echo "👍 Good progress. Address failed tests and rerun." >> "$REPORT_FILE"
    else
        echo "⚠️ Multiple failures. Review logs and fix critical issues." >> "$REPORT_FILE"
    fi
    
    cat >> "$REPORT_FILE" <<EOF

### Next Steps

1. Review test logs in \`$LOG_DIR/\`
2. Fix any failed tests
3. Rerun tests: \`./test_all_demos.sh\`
4. Update documentation if needed

---

## 📚 References

- [COMPREHENSIVE_TEST_PLAN.md](COMPREHENSIVE_TEST_PLAN.md) - Full test plan
- [00_START_HERE.md](00_START_HERE.md) - Showcase entry
- [../STATUS.md](../STATUS.md) - Overall status

---

**Test Complete!**
EOF

    log_pass "Report generated: $REPORT_FILE"
}

# =============================================================================
# Main Execution
# =============================================================================

main() {
    log_header "NestGate Showcase Comprehensive Test Suite"
    
    echo "Configuration:"
    echo "  NestGate URL:    $NESTGATE_URL"
    echo "  Test Mode:       $TEST_MODE"
    echo "  Log Directory:   $LOG_DIR"
    echo "  Report File:     $REPORT_FILE"
    echo ""
    
    # Prerequisites
    if ! check_prerequisites; then
        log_error "Missing required prerequisites"
        exit 1
    fi
    
    # Service check
    if ! check_nestgate_service; then
        log_error "NestGate service must be running"
        log_info "Start with: cd $NESTGATE_ROOT && ./start_local_dev.sh"
        exit 1
    fi
    
    # Run tests
    test_level_1
    test_level_2
    test_level_3
    
    # Generate report
    generate_report
    
    # Summary
    log_header "Test Summary"
    echo "Tests Total:    $TESTS_TOTAL"
    echo -e "Tests Passed:   ${GREEN}$TESTS_PASSED${NC} ✓"
    echo -e "Tests Failed:   ${RED}$TESTS_FAILED${NC} ✗"
    echo -e "Tests Skipped:  ${YELLOW}$TESTS_SKIPPED${NC} ⊘"
    echo ""
    
    local pass_rate=0
    [ $TESTS_TOTAL -gt 0 ] && pass_rate=$((TESTS_PASSED * 100 / TESTS_TOTAL))
    echo "Pass Rate:      ${pass_rate}%"
    echo ""
    echo "Duration:       $(($(date +%s) - START_TIME))s"
    echo ""
    echo "Report:         $REPORT_FILE"
    echo "Logs:           $LOG_DIR/"
    echo ""
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_pass "All tests completed successfully!"
        return 0
    else
        log_fail "$TESTS_FAILED test(s) failed"
        return 1
    fi
}

# Run main function
main "$@"
