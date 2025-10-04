#!/bin/bash
# 🚀 PRODUCTION VALIDATION SCRIPT
# Comprehensive validation for production deployment readiness

set -euo pipefail

echo "🚀 **NESTGATE PRODUCTION VALIDATION**"
echo "===================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Validation results
VALIDATION_PASSED=0
VALIDATION_FAILED=0

# Function to run validation check
validate_check() {
    local description="$1"
    local command="$2"
    
    echo -e "${BLUE}🔍 Validating: $description${NC}"
    
    if eval "$command" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ PASSED: $description${NC}"
        ((VALIDATION_PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAILED: $description${NC}"
        ((VALIDATION_FAILED++))
        return 1
    fi
}

# Function to run validation with output
validate_with_output() {
    local description="$1"
    local command="$2"
    
    echo -e "${BLUE}🔍 Validating: $description${NC}"
    
    if eval "$command"; then
        echo -e "${GREEN}✅ PASSED: $description${NC}"
        ((VALIDATION_PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAILED: $description${NC}"
        ((VALIDATION_FAILED++))
        return 1
    fi
}

echo -e "${YELLOW}📋 **CORE VALIDATION SUITE**${NC}"
echo "=============================="

# Core build validation
validate_check "Clean workspace compilation" "cargo check --workspace --message-format short"
validate_check "Release build compilation" "timeout 60 cargo build --release --workspace --message-format short"
validate_check "Core package compilation" "cargo check --package nestgate-core --message-format short"
validate_check "API package compilation" "cargo check --package nestgate-api --message-format short"

echo ""
echo -e "${YELLOW}🧪 **TEST SUITE VALIDATION**${NC}"
echo "============================"

# Test validation
validate_check "Unit tests execution" "timeout 120 cargo test --workspace --lib --message-format short"
validate_check "Integration tests" "timeout 60 cargo test --package nestgate-core --message-format short"

echo ""
echo -e "${YELLOW}⚡ **PERFORMANCE VALIDATION**${NC}"
echo "============================="

# Performance validation
validate_check "Benchmark compilation" "timeout 30 cargo bench --no-run --workspace --message-format short"

echo ""
echo -e "${YELLOW}📊 **CODE QUALITY VALIDATION**${NC}"
echo "=============================="

# Code quality checks
validate_check "Clippy linting" "cargo clippy --workspace --all-targets -- -D warnings"
validate_check "Format checking" "cargo fmt --all -- --check"

echo ""
echo -e "${YELLOW}🔍 **ARCHITECTURE VALIDATION**${NC}"
echo "=============================="

# Architecture validation
validate_check "Unified constants exist" "test -f code/crates/nestgate-core/src/constants/unified_canonical.rs"
validate_check "Error system unified" "grep -q 'pub enum NestGateError' code/crates/nestgate-core/src/error/mod.rs"
validate_check "Configuration unified" "test -f code/crates/nestgate-core/src/unified_config_master.rs"

echo ""
echo -e "${YELLOW}📁 **FILE SIZE COMPLIANCE**${NC}"
echo "=========================="

# File size validation
MAX_LINES=2000
LARGE_FILES=$(find code/crates -name "*.rs" -exec wc -l {} + | awk -v max=$MAX_LINES '$1 > max {print $2 " (" $1 " lines)"}' | head -5)

if [ -z "$LARGE_FILES" ]; then
    echo -e "${GREEN}✅ PASSED: All files under 2,000 lines${NC}"
    ((VALIDATION_PASSED++))
else
    echo -e "${RED}❌ FAILED: Files exceed 2,000 lines:${NC}"
    echo "$LARGE_FILES"
    ((VALIDATION_FAILED++))
fi

echo ""
echo -e "${YELLOW}📈 **FINAL VALIDATION REPORT**${NC}"
echo "============================="

TOTAL_CHECKS=$((VALIDATION_PASSED + VALIDATION_FAILED))
SUCCESS_RATE=$((VALIDATION_PASSED * 100 / TOTAL_CHECKS))

echo "Total Validation Checks: $TOTAL_CHECKS"
echo -e "${GREEN}Passed: $VALIDATION_PASSED${NC}"
echo -e "${RED}Failed: $VALIDATION_FAILED${NC}"
echo "Success Rate: $SUCCESS_RATE%"

echo ""
if [ $VALIDATION_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 **VALIDATION COMPLETE - PRODUCTION READY!**${NC}"
    echo -e "${GREEN}✅ All validation checks passed successfully${NC}"
    echo -e "${GREEN}🚀 Ready for production deployment${NC}"
    exit 0
else
    echo -e "${RED}⚠️  **VALIDATION ISSUES DETECTED**${NC}"
    echo -e "${RED}❌ $VALIDATION_FAILED validation checks failed${NC}"
    echo -e "${YELLOW}🔧 Please address issues before production deployment${NC}"
    exit 1
fi 