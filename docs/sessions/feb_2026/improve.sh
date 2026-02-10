#!/usr/bin/env bash
# 🚀 NestGate Systematic Improvement Automation Script
# December 14, 2025
# 
# This script assists with the systematic improvement plan execution.
# Run individual phases or the full improvement cycle.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root (assumes script is in project root)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  NestGate Systematic Improvement - Automation Script${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# Function to print section headers
section_header() {
    echo -e "\n${GREEN}▶ $1${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Function to print status
status() {
    echo -e "${YELLOW}  → $1${NC}"
}

# Function to print success
success() {
    echo -e "${GREEN}  ✓ $1${NC}"
}

# Function to print error
error() {
    echo -e "${RED}  ✗ $1${NC}"
}

# ============================================================================
# PHASE 1: Quality Checks
# ============================================================================
phase1_quality_checks() {
    section_header "Phase 1: Quality Checks"
    
    status "Running cargo fmt check..."
    if cargo fmt --check; then
        success "Formatting is correct"
    else
        error "Formatting issues found. Run: cargo fmt"
        return 1
    fi
    
    status "Running cargo clippy..."
    cargo clippy --workspace --all-targets 2>&1 | tee clippy_output.log
    success "Clippy analysis complete (see clippy_output.log)"
    
    status "Running cargo test..."
    cargo test --lib --workspace 2>&1 | tee test_output.log
    
    # Extract test results
    PASS_COUNT=$(grep "test result:" test_output.log | grep -oP '\d+(?= passed)' | head -1 || echo "0")
    FAIL_COUNT=$(grep "test result:" test_output.log | grep -oP '\d+(?= failed)' | head -1 || echo "0")
    
    echo ""
    success "Tests complete: ${PASS_COUNT} passed, ${FAIL_COUNT} failed"
    
    if [ "${FAIL_COUNT}" -gt 0 ]; then
        error "Some tests failed. Review test_output.log"
    fi
}

# ============================================================================
# PHASE 2: Find Hardcoded Values
# ============================================================================
phase2_find_hardcoding() {
    section_header "Phase 2: Finding Hardcoded Values"
    
    status "Searching for hardcoded IPs..."
    grep -rn "localhost\|127\.0\.0\.1\|0\.0\.0\.0" code/crates --include="*.rs" \
        | grep -v test \
        | grep -v "^Binary" \
        > hardcoded_ips.txt 2>/dev/null || true
    
    IP_COUNT=$(wc -l < hardcoded_ips.txt)
    success "Found ${IP_COUNT} hardcoded IP references (production)"
    
    status "Searching for hardcoded ports..."
    grep -rn ":[0-9]\{4,5\}" code/crates --include="*.rs" \
        | grep -v test \
        | grep -v "^Binary" \
        > hardcoded_ports.txt 2>/dev/null || true
    
    PORT_COUNT=$(wc -l < hardcoded_ports.txt)
    success "Found ${PORT_COUNT} hardcoded port references (production)"
    
    echo ""
    status "Priority migration targets saved to:"
    echo "  - hardcoded_ips.txt"
    echo "  - hardcoded_ports.txt"
}

# ============================================================================
# PHASE 3: Find Unwraps
# ============================================================================
phase3_find_unwraps() {
    section_header "Phase 3: Finding Unwraps"
    
    status "Searching for unwrap() in production code..."
    grep -rn "\.unwrap()" code/crates/*/src --include="*.rs" \
        | grep -v "// Test:" \
        | grep -v "test_" \
        > production_unwraps.txt 2>/dev/null || true
    
    UNWRAP_COUNT=$(wc -l < production_unwraps.txt)
    success "Found ${UNWRAP_COUNT} unwrap() calls in production"
    
    status "Searching for expect() in production code..."
    grep -rn "\.expect(" code/crates/*/src --include="*.rs" \
        | grep -v "// Test:" \
        | grep -v "test_" \
        > production_expects.txt 2>/dev/null || true
    
    EXPECT_COUNT=$(wc -l < production_expects.txt)
    success "Found ${EXPECT_COUNT} expect() calls in production"
    
    echo ""
    status "Priority unwrap targets saved to:"
    echo "  - production_unwraps.txt"
    echo "  - production_expects.txt"
}

# ============================================================================
# PHASE 4: Measure Coverage
# ============================================================================
phase4_measure_coverage() {
    section_header "Phase 4: Measuring Test Coverage"
    
    if ! command -v cargo-llvm-cov &> /dev/null; then
        error "cargo-llvm-cov not found. Install with:"
        echo "  cargo install cargo-llvm-cov"
        return 1
    fi
    
    status "Generating coverage report (this may take a while)..."
    cargo llvm-cov --workspace --lib --html 2>&1 | tee coverage_output.log || {
        error "Coverage measurement failed (likely due to failing tests)"
        echo "  Fix failing tests first, then re-run coverage"
        return 1
    }
    
    success "Coverage report generated in target/llvm-cov/html/"
    echo "  Open: target/llvm-cov/html/index.html"
    
    # Try to extract coverage percentage
    if command -v cargo-llvm-cov &> /dev/null; then
        cargo llvm-cov --workspace --lib --json --output-path coverage.json 2>/dev/null || true
        if [ -f coverage.json ]; then
            success "JSON coverage data: coverage.json"
        fi
    fi
}

# ============================================================================
# PHASE 5: Generate Report
# ============================================================================
phase5_generate_report() {
    section_header "Phase 5: Generating Progress Report"
    
    REPORT_FILE="EXECUTION_PROGRESS_$(date +%Y_%m_%d).txt"
    
    {
        echo "═══════════════════════════════════════════════════════════"
        echo "  NestGate Improvement Execution Report"
        echo "  Generated: $(date)"
        echo "═══════════════════════════════════════════════════════════"
        echo ""
        
        echo "QUALITY CHECKS:"
        echo "  - Formatting: $(cargo fmt --check 2>&1 >/dev/null && echo 'PASS' || echo 'FAIL')"
        echo "  - Tests: ${PASS_COUNT:-0} passed, ${FAIL_COUNT:-0} failed"
        echo ""
        
        echo "HARDCODING ANALYSIS:"
        echo "  - Hardcoded IPs (production): ${IP_COUNT:-0}"
        echo "  - Hardcoded ports (production): ${PORT_COUNT:-0}"
        echo ""
        
        echo "UNWRAP ANALYSIS:"
        echo "  - unwrap() calls (production): ${UNWRAP_COUNT:-0}"
        echo "  - expect() calls (production): ${EXPECT_COUNT:-0}"
        echo ""
        
        echo "FILES GENERATED:"
        echo "  - hardcoded_ips.txt"
        echo "  - hardcoded_ports.txt"
        echo "  - production_unwraps.txt"
        echo "  - production_expects.txt"
        echo "  - clippy_output.log"
        echo "  - test_output.log"
        [ -f coverage.json ] && echo "  - coverage.json"
        echo ""
        
        echo "NEXT STEPS:"
        echo "  1. Review generated files for migration targets"
        echo "  2. Use capability_based.rs for hardcoding migration"
        echo "  3. Use safe_operations.rs for unwrap replacement"
        echo "  4. Add tests for error paths"
        echo "  5. Re-run this script to track progress"
        echo ""
        
        echo "DETAILED REPORTS:"
        echo "  - COMPREHENSIVE_AUDIT_REPORT_DEC_14_2025.md"
        echo "  - AUDIT_EXECUTIVE_SUMMARY_DEC_14_2025.md"
        echo ""
    } | tee "${REPORT_FILE}"
    
    success "Report generated: ${REPORT_FILE}"
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

# Parse command line arguments
PHASE="${1:-all}"

case "$PHASE" in
    "1"|"quality")
        phase1_quality_checks
        ;;
    "2"|"hardcoding")
        phase2_find_hardcoding
        ;;
    "3"|"unwraps")
        phase3_find_unwraps
        ;;
    "4"|"coverage")
        phase4_measure_coverage
        ;;
    "5"|"report")
        phase5_generate_report
        ;;
    "all")
        phase1_quality_checks || true
        phase2_find_hardcoding
        phase3_find_unwraps
        phase4_measure_coverage || true
        phase5_generate_report
        ;;
    "help"|"--help"|"-h")
        echo "Usage: $0 [phase]"
        echo ""
        echo "Phases:"
        echo "  1, quality    - Run quality checks (fmt, clippy, tests)"
        echo "  2, hardcoding - Find hardcoded values"
        echo "  3, unwraps    - Find unwrap() and expect() calls"
        echo "  4, coverage   - Measure test coverage"
        echo "  5, report     - Generate progress report"
        echo "  all           - Run all phases (default)"
        echo "  help          - Show this help"
        echo ""
        echo "Examples:"
        echo "  $0              # Run all phases"
        echo "  $0 quality      # Run only quality checks"
        echo "  $0 hardcoding   # Find hardcoded values only"
        ;;
    *)
        error "Unknown phase: $PHASE"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac

echo ""
section_header "Execution Complete"
success "All requested phases completed!"
echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"

