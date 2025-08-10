#!/bin/bash
# NestGate Comprehensive Test Coverage Analysis
# This script provides detailed test coverage analysis for the entire NestGate codebase

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
COVERAGE_DIR="./target/coverage"
REPORT_DIR="./coverage-reports"
TARGET_COVERAGE=90

echo -e "${BLUE}🧪 NestGate Comprehensive Test Coverage Analysis${NC}"
echo -e "${BLUE}=================================================${NC}"

# Create directories
mkdir -p "$COVERAGE_DIR"
mkdir -p "$REPORT_DIR"

# Clean previous coverage data
echo -e "${YELLOW}🧹 Cleaning previous coverage data...${NC}"
cargo clean
rm -rf "$COVERAGE_DIR"/*
rm -rf "$REPORT_DIR"/*

# Set environment variables for coverage
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="$COVERAGE_DIR/nestgate-%p-%m.profraw"

echo -e "${CYAN}📊 Running tests with coverage instrumentation...${NC}"

# Function to run tests for a specific crate with coverage
run_crate_tests() {
    local crate_name=$1
    local crate_path="code/crates/$crate_name"
    
    if [ -d "$crate_path" ]; then
        echo -e "${PURPLE}📦 Testing $crate_name...${NC}"
        cd "$crate_path"
        
        # Run unit tests
        cargo test --lib 2>/dev/null || echo -e "${RED}⚠️  Some unit tests failed in $crate_name${NC}"
        
        # Run integration tests if they exist
        if [ -d "tests" ]; then
            cargo test --tests 2>/dev/null || echo -e "${RED}⚠️  Some integration tests failed in $crate_name${NC}"
        fi
        
        cd - > /dev/null
    else
        echo -e "${RED}❌ Crate $crate_name not found${NC}"
    fi
}

# Run tests for all NestGate crates
CRATES=(
    "nestgate-core"
    "nestgate-api"
    "nestgate-automation"
    "nestgate-bin"
    "nestgate-fsmonitor"
    "nestgate-installer"
    "nestgate-mcp"
    "nestgate-middleware"
    "nestgate-nas"
    "nestgate-network"
    "nestgate-zfs"
)

for crate in "${CRATES[@]}"; do
    run_crate_tests "$crate"
done

# Run workspace-level tests
echo -e "${PURPLE}🏗️  Running workspace-level tests...${NC}"
cargo test --workspace --exclude fuzz 2>/dev/null || echo -e "${RED}⚠️  Some workspace tests failed${NC}"

# Run integration tests
echo -e "${PURPLE}🔗 Running integration tests...${NC}"
if [ -d "tests" ]; then
    cargo test --tests 2>/dev/null || echo -e "${RED}⚠️  Some integration tests failed${NC}"
fi

# Generate coverage report
echo -e "${CYAN}📈 Generating coverage reports...${NC}"

# Check if we have any coverage data
if [ -z "$(ls -A $COVERAGE_DIR/*.profraw 2>/dev/null)" ]; then
    echo -e "${RED}❌ No coverage data found. Tests may have failed to run with instrumentation.${NC}"
    exit 1
fi

# Generate HTML report
grcov "$COVERAGE_DIR" \
    --source-dir . \
    --binary-path ./target/debug \
    --output-type html \
    --output-path "$REPORT_DIR/html" \
    --branch \
    --ignore-not-existing \
    --ignore "/*" \
    --ignore "target/*" \
    --ignore "tests/*" \
    --ignore "benches/*" \
    --ignore "examples/*" \
    --ignore "*/tests/*" \
    --ignore "*/benches/*" \
    --ignore "*/examples/*"

# Generate LCOV report for CI/CD integration
grcov "$COVERAGE_DIR" \
    --source-dir . \
    --binary-path ./target/debug \
    --output-type lcov \
    --output-path "$REPORT_DIR/lcov.info" \
    --branch \
    --ignore-not-existing \
    --ignore "/*" \
    --ignore "target/*" \
    --ignore "tests/*" \
    --ignore "benches/*" \
    --ignore "examples/*" \
    --ignore "*/tests/*" \
    --ignore "*/benches/*" \
    --ignore "*/examples/*"

# Generate summary report (skip coveralls format due to token requirement)
echo -e "${CYAN}📊 Generating text summary...${NC}"
grcov "$COVERAGE_DIR" \
    --source-dir . \
    --binary-path ./target/debug \
    --output-type summary \
    --output-path "$REPORT_DIR/summary.txt" \
    --branch \
    --ignore-not-existing \
    --ignore "/*" \
    --ignore "target/*" \
    --ignore "tests/*" \
    --ignore "benches/*" \
    --ignore "examples/*" \
    --ignore "*/tests/*" \
    --ignore "*/benches/*" \
    --ignore "*/examples/*"

# Extract coverage percentage from summary report
if [ -f "$REPORT_DIR/summary.txt" ]; then
    # Extract coverage percentage from summary text
    COVERAGE_PERCENT=$(grep -o '[0-9]*\.[0-9]*%' "$REPORT_DIR/summary.txt" | head -1 | sed 's/%//' 2>/dev/null || echo "0")
    
    # If that fails, try extracting from HTML report
    if [ "$COVERAGE_PERCENT" = "0" ] && [ -f "$REPORT_DIR/html/index.html" ]; then
        COVERAGE_PERCENT=$(grep -o '[0-9]*\.[0-9]*%' "$REPORT_DIR/html/index.html" | head -1 | sed 's/%//' 2>/dev/null || echo "0")
    fi
else
    COVERAGE_PERCENT="0"
fi

# Round coverage percentage
COVERAGE_PERCENT=$(printf "%.1f" "$COVERAGE_PERCENT" 2>/dev/null || echo "0.0")

echo -e "${BLUE}📊 Coverage Analysis Complete${NC}"
echo -e "${BLUE}=============================${NC}"
echo -e "${CYAN}📁 HTML Report: $REPORT_DIR/html/index.html${NC}"
echo -e "${CYAN}📄 LCOV Report: $REPORT_DIR/lcov.info${NC}"
echo -e "${CYAN}📋 Summary Report: $REPORT_DIR/summary.txt${NC}"

# Coverage summary
echo ""
echo -e "${PURPLE}📈 COVERAGE SUMMARY${NC}"
echo -e "${PURPLE}===================${NC}"
echo -e "${CYAN}Current Coverage: ${COVERAGE_PERCENT}%${NC}"
echo -e "${CYAN}Target Coverage:  ${TARGET_COVERAGE}%${NC}"

# Coverage assessment
COVERAGE_INT=$(printf "%.0f" "$COVERAGE_PERCENT" 2>/dev/null || echo "0")
if [ "$COVERAGE_INT" -ge "$TARGET_COVERAGE" ]; then
    echo -e "${GREEN}✅ Coverage target achieved!${NC}"
    COVERAGE_STATUS="PASS"
elif [ "$COVERAGE_INT" -ge 70 ]; then
    echo -e "${YELLOW}⚠️  Coverage approaching target (good progress)${NC}"
    COVERAGE_STATUS="WARN"
else
    echo -e "${RED}❌ Coverage below target (needs improvement)${NC}"
    COVERAGE_STATUS="FAIL"
fi

# Generate detailed analysis
echo ""
echo -e "${PURPLE}🔍 DETAILED ANALYSIS${NC}"
echo -e "${PURPLE}===================${NC}"

# Count test files
UNIT_TESTS=$(find . -name "*test*.rs" -o -name "tests.rs" | grep -v target | wc -l)
INTEGRATION_TESTS=$(find tests/ -name "*.rs" 2>/dev/null | wc -l || echo "0")
TOTAL_TESTS=$(($UNIT_TESTS + $INTEGRATION_TESTS))

echo -e "${CYAN}📝 Unit Test Files: $UNIT_TESTS${NC}"
echo -e "${CYAN}🔗 Integration Test Files: $INTEGRATION_TESTS${NC}"
echo -e "${CYAN}📊 Total Test Files: $TOTAL_TESTS${NC}"

# Count source files
SOURCE_FILES=$(find code/crates/ -name "*.rs" | grep -v target | grep -v "/tests/" | wc -l)
echo -e "${CYAN}📄 Source Files: $SOURCE_FILES${NC}"

# Test-to-source ratio
if [ "$SOURCE_FILES" -gt 0 ]; then
    TEST_RATIO=$(echo "scale=2; $TOTAL_TESTS / $SOURCE_FILES" | bc -l 2>/dev/null || echo "0")
    echo -e "${CYAN}📏 Test-to-Source Ratio: $TEST_RATIO${NC}"
fi

# Recommendations
echo ""
echo -e "${PURPLE}💡 RECOMMENDATIONS${NC}"
echo -e "${PURPLE}==================${NC}"

if [ "$COVERAGE_STATUS" = "FAIL" ]; then
    echo -e "${YELLOW}🎯 Priority Actions:${NC}"
    echo -e "   • Add unit tests for core business logic"
    echo -e "   • Implement integration tests for API endpoints"
    echo -e "   • Add error path testing"
    echo -e "   • Test edge cases and boundary conditions"
elif [ "$COVERAGE_STATUS" = "WARN" ]; then
    echo -e "${YELLOW}🎯 Improvement Actions:${NC}"
    echo -e "   • Add tests for remaining uncovered code paths"
    echo -e "   • Implement property-based testing"
    echo -e "   • Add chaos engineering tests"
    echo -e "   • Enhance error handling tests"
else
    echo -e "${GREEN}🎯 Maintenance Actions:${NC}"
    echo -e "   • Maintain current coverage levels"
    echo -e "   • Add tests for new features"
    echo -e "   • Consider mutation testing"
    echo -e "   • Implement performance regression tests"
fi

# Additional test types to consider
echo ""
echo -e "${YELLOW}🧪 Additional Testing Opportunities:${NC}"
echo -e "   • Chaos Engineering (fault injection)"
echo -e "   • Property-Based Testing (quickcheck)"
echo -e "   • Mutation Testing (cargo-mutants)"
echo -e "   • Performance Testing (criterion)"
echo -e "   • Security Testing (penetration tests)"
echo -e "   • Compliance Testing (regulatory requirements)"

# Generate summary file
cat > "$REPORT_DIR/summary.md" << EOF
# NestGate Test Coverage Summary

**Generated:** $(date)
**Coverage:** ${COVERAGE_PERCENT}%
**Target:** ${TARGET_COVERAGE}%
**Status:** ${COVERAGE_STATUS}

## Statistics
- **Unit Test Files:** ${UNIT_TESTS}
- **Integration Test Files:** ${INTEGRATION_TESTS}
- **Total Test Files:** ${TOTAL_TESTS}
- **Source Files:** ${SOURCE_FILES}
- **Test-to-Source Ratio:** ${TEST_RATIO:-N/A}

## Reports
- [HTML Report](html/index.html)
- [LCOV Report](lcov.info)
- [Summary Report](summary.txt)

## Status
EOF

if [ "$COVERAGE_STATUS" = "PASS" ]; then
    echo "✅ **PASS** - Coverage target achieved" >> "$REPORT_DIR/summary.md"
elif [ "$COVERAGE_STATUS" = "WARN" ]; then
    echo "⚠️ **WARNING** - Coverage approaching target" >> "$REPORT_DIR/summary.md"
else
    echo "❌ **FAIL** - Coverage below target" >> "$REPORT_DIR/summary.md"
fi

echo ""
echo -e "${GREEN}🎉 Coverage analysis complete!${NC}"
echo -e "${CYAN}📁 Open $REPORT_DIR/html/index.html in your browser to view detailed results${NC}"

# Exit with appropriate code
if [ "$COVERAGE_STATUS" = "FAIL" ]; then
    exit 1
else
    exit 0
fi 