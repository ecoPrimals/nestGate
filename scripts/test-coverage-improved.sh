#!/bin/bash
# **ENHANCED TEST COVERAGE ANALYSIS - 50% TARGET**
# 
# Comprehensive test coverage measurement and improvement script

set -euo pipefail

echo "🎯 **NESTGATE ENHANCED TEST COVERAGE ANALYSIS**"
echo "Target: 50% Code Coverage"
echo "=========================================="

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
COVERAGE_TARGET=50
COVERAGE_DIR="coverage-enhanced"
REPORT_DIR="coverage-reports/enhanced"

# Create directories
mkdir -p "$COVERAGE_DIR"
mkdir -p "$REPORT_DIR"

echo "📊 Installing coverage tools..."
cargo install cargo-tarpaulin 2>/dev/null || echo "cargo-tarpaulin already installed"
cargo install cargo-llvm-cov 2>/dev/null || echo "cargo-llvm-cov already installed"

echo ""
echo "🧪 Running comprehensive test suite with coverage..."

# Run tests with coverage using tarpaulin
echo "📈 Generating coverage with cargo-tarpaulin..."
cargo tarpaulin \
    --workspace \
    --all-features \
    --timeout 300 \
    --skip-clean \
    --engine llvm \
    --out Html \
    --out Lcov \
    --out Json \
    --output-dir "$COVERAGE_DIR" \
    --exclude-files "target/*" \
    --exclude-files "tests/*" \
    --exclude-files "benches/*" \
    --exclude-files "examples/*" \
    --exclude-files "fuzz/*" \
    --line \
    --branch \
    --count \
    2>&1 | tee "$REPORT_DIR/tarpaulin-output.log"

# Extract coverage percentage
COVERAGE_PERCENT=$(grep -o '[0-9]\+\.[0-9]\+%' "$REPORT_DIR/tarpaulin-output.log" | tail -1 | sed 's/%//')

echo ""
echo "📊 **COVERAGE ANALYSIS RESULTS**"
echo "================================"

if (( $(echo "$COVERAGE_PERCENT >= $COVERAGE_TARGET" | bc -l) )); then
    echo -e "${GREEN}✅ COVERAGE TARGET ACHIEVED: $COVERAGE_PERCENT% (Target: $COVERAGE_TARGET%)${NC}"
    COVERAGE_STATUS="PASSED"
else
    echo -e "${YELLOW}⚠️  COVERAGE BELOW TARGET: $COVERAGE_PERCENT% (Target: $COVERAGE_TARGET%)${NC}"
    COVERAGE_STATUS="NEEDS_IMPROVEMENT"
fi

echo ""
echo "🔍 **DETAILED COVERAGE BREAKDOWN**"
echo "================================="

# Analyze coverage by crate
echo "📦 Coverage by crate:"
if [ -f "$COVERAGE_DIR/lcov.info" ]; then
    # Parse lcov.info for per-file coverage
    awk '
    /^SF:/ { file = $0; gsub(/^SF:/, "", file) }
    /^LH:/ { lines_hit = $0; gsub(/^LH:/, "", lines_hit) }
    /^LF:/ { 
        lines_found = $0; gsub(/^LF:/, "", lines_found)
        if (lines_found > 0) {
            coverage = (lines_hit / lines_found) * 100
            gsub(/.*\/code\/crates\//, "", file)
            gsub(/\/.*/, "", file)
            crates[file] += coverage
            crate_counts[file]++
        }
    }
    END {
        for (crate in crates) {
            avg_coverage = crates[crate] / crate_counts[crate]
            printf "  %-25s: %6.2f%%\n", crate, avg_coverage
        }
    }
    ' "$COVERAGE_DIR/lcov.info"
fi

echo ""
echo "🎯 **COVERAGE IMPROVEMENT OPPORTUNITIES**"
echo "========================================"

# Identify uncovered areas
if [ -f "$COVERAGE_DIR/tarpaulin-report.json" ]; then
    echo "📝 Files with lowest coverage (need attention):"
    jq -r '.files[] | select(.coverage < 0.5) | "\(.name): \(.coverage * 100 | floor)%"' \
        "$COVERAGE_DIR/tarpaulin-report.json" 2>/dev/null | head -10 || echo "  No JSON data available"
fi

echo ""
echo "🧪 **TEST SUITE STATISTICS**"
echo "============================"

# Count test types
UNIT_TESTS=$(find tests/unit -name "*.rs" | wc -l)
INTEGRATION_TESTS=$(find tests/integration -name "*.rs" | wc -l)
E2E_TESTS=$(find tests/e2e -name "*.rs" | wc -l)
CHAOS_TESTS=$(find tests/chaos -name "*.rs" | wc -l)
BENCHMARK_TESTS=$(find benches -name "*.rs" | wc -l)

echo "  Unit Tests:        $UNIT_TESTS files"
echo "  Integration Tests: $INTEGRATION_TESTS files"
echo "  E2E Tests:         $E2E_TESTS files"
echo "  Chaos Tests:       $CHAOS_TESTS files"
echo "  Benchmark Tests:   $BENCHMARK_TESTS files"

# Calculate total test functions
TOTAL_TEST_FUNCTIONS=$(grep -r "#\[test\]" tests/ --include="*.rs" | wc -l)
TOTAL_ASYNC_TESTS=$(grep -r "#\[tokio::test\]" tests/ --include="*.rs" | wc -l)

echo "  Total Test Functions: $TOTAL_TEST_FUNCTIONS"
echo "  Async Test Functions: $TOTAL_ASYNC_TESTS"

echo ""
echo "📈 **COVERAGE IMPROVEMENT RECOMMENDATIONS**"
echo "=========================================="

if [ "$COVERAGE_STATUS" = "NEEDS_IMPROVEMENT" ]; then
    echo "🎯 To reach 50% coverage target:"
    COVERAGE_GAP=$(echo "$COVERAGE_TARGET - $COVERAGE_PERCENT" | bc -l)
    echo "  • Coverage gap to close: ${COVERAGE_GAP}%"
    echo "  • Focus on adding tests for uncovered modules"
    echo "  • Prioritize core business logic testing"
    echo "  • Add integration tests for service interactions"
    echo "  • Implement property-based testing for critical paths"
fi

echo ""
echo "🔧 **SUGGESTED NEXT ACTIONS**"
echo "============================"
echo "1. Add unit tests for uncovered core modules"
echo "2. Implement integration tests for service workflows"
echo "3. Add property-based tests for critical algorithms"
echo "4. Implement chaos testing for fault tolerance"
echo "5. Add performance regression tests"

echo ""
echo "📁 **COVERAGE REPORTS GENERATED**"
echo "================================"
echo "  HTML Report: $COVERAGE_DIR/tarpaulin-report.html"
echo "  LCOV Report: $COVERAGE_DIR/lcov.info"
echo "  JSON Report: $COVERAGE_DIR/tarpaulin-report.json"

# Generate summary report
cat > "$REPORT_DIR/coverage-summary.md" << EOF
# Test Coverage Summary Report

**Generated**: $(date)
**Coverage**: $COVERAGE_PERCENT%
**Target**: $COVERAGE_TARGET%
**Status**: $COVERAGE_STATUS

## Test Suite Statistics
- Unit Tests: $UNIT_TESTS files
- Integration Tests: $INTEGRATION_TESTS files  
- E2E Tests: $E2E_TESTS files
- Chaos Tests: $CHAOS_TESTS files
- Total Test Functions: $TOTAL_TEST_FUNCTIONS

## Coverage Analysis
- Current Coverage: $COVERAGE_PERCENT%
- Target Coverage: $COVERAGE_TARGET%
- Status: $COVERAGE_STATUS

## Next Actions
1. Focus on core module testing
2. Add service integration tests
3. Implement property-based testing
4. Add chaos and fault tolerance tests
EOF

echo ""
echo -e "${BLUE}📋 Coverage summary saved to: $REPORT_DIR/coverage-summary.md${NC}"

if [ "$COVERAGE_STATUS" = "PASSED" ]; then
    echo -e "${GREEN}🎉 **COVERAGE TARGET ACHIEVED!**${NC}"
    exit 0
else
    echo -e "${YELLOW}📈 **COVERAGE IMPROVEMENT NEEDED**${NC}"
    echo "Run this script again after adding tests to track progress."
    exit 1
fi 