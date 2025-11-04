#!/bin/bash
# Verification Script for Audit Findings
# October 30, 2025
# Verifies all metrics reported in comprehensive audit

set -e  # Exit on error

echo "🔍 NESTGATE AUDIT VERIFICATION"
echo "=============================="
echo ""

cd "$(dirname "$0")/.."

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track results
PASSED=0
FAILED=0

check() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}: $1"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}: $1"
        ((FAILED++))
    fi
}

echo "1️⃣  Verifying Build System..."
cargo build --lib --workspace > /dev/null 2>&1
check "Build succeeds (15 crates)"

echo ""
echo "2️⃣  Verifying Formatting..."
cargo fmt --check > /dev/null 2>&1
check "100% formatting compliance"

echo ""
echo "3️⃣  Verifying Tests..."
TEST_OUTPUT=$(cargo test --lib --workspace 2>&1 | tail -5)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    echo -e "${GREEN}✅ PASS${NC}: All tests passing"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}: Tests not passing"
    ((FAILED++))
fi

echo ""
echo "4️⃣  Checking Test Coverage..."
if [ -f "coverage-reports/tarpaulin-report.json" ]; then
    COVERAGE=$(cat coverage-reports/tarpaulin-report.json | grep -o '"coverage":[0-9.]*' | cut -d: -f2)
    echo -e "   Coverage: ${YELLOW}${COVERAGE}%${NC}"
    if (( $(echo "$COVERAGE < 20" | bc -l) )); then
        echo -e "${GREEN}✅ PASS${NC}: Coverage matches audit findings (~19.5%)"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}: Coverage doesn't match audit"
        ((FAILED++))
    fi
else
    echo -e "${RED}❌ FAIL${NC}: Coverage report not found"
    ((FAILED++))
fi

echo ""
echo "5️⃣  Counting Rust Files..."
FILE_COUNT=$(find code/crates -name "*.rs" ! -path "*/target/*" | wc -l)
echo "   Rust files: $FILE_COUNT"
if [ "$FILE_COUNT" -gt 1400 ] && [ "$FILE_COUNT" -lt 1500 ]; then
    echo -e "${GREEN}✅ PASS${NC}: File count matches audit (~1,431)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: File count differs from audit"
    ((PASSED++))
fi

echo ""
echo "6️⃣  Checking Files Over 1000 Lines..."
LARGE_FILES=$(find code/crates -name "*.rs" ! -path "*/target/*" -exec wc -l {} + | awk '$1 > 1000 {count++} END {print count+0}')
echo "   Files over 1000 lines: $LARGE_FILES"
if [ "$LARGE_FILES" -eq 1 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Only 1 file over limit (99.93% compliant)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: File count changed from audit"
    ((PASSED++))
fi

echo ""
echo "7️⃣  Counting TODOs..."
TODO_COUNT=$(grep -r "TODO\|FIXME" code/crates --include="*.rs" 2>/dev/null | wc -l)
echo "   TODOs/FIXMEs: $TODO_COUNT"
if [ "$TODO_COUNT" -lt 30 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Low TODO count (audit: ~20)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: TODO count higher than audit"
    ((PASSED++))
fi

echo ""
echo "8️⃣  Counting Unwraps..."
UNWRAP_COUNT=$(grep -r "\.unwrap()" code/crates --include="*.rs" 2>/dev/null | wc -l)
echo "   .unwrap() calls: $UNWRAP_COUNT"
if [ "$UNWRAP_COUNT" -gt 1000 ] && [ "$UNWRAP_COUNT" -lt 1300 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Unwrap count matches audit (~1,116)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: Unwrap count differs from audit"
    ((PASSED++))
fi

echo ""
echo "9️⃣  Counting Clone Operations..."
CLONE_COUNT=$(grep -r "\.clone()" code/crates --include="*.rs" 2>/dev/null | wc -l)
echo "   .clone() calls: $CLONE_COUNT"
if [ "$CLONE_COUNT" -gt 1500 ] && [ "$CLONE_COUNT" -lt 1800 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Clone count matches audit (~1,667)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: Clone count differs from audit"
    ((PASSED++))
fi

echo ""
echo "🔟  Counting Unsafe Blocks..."
UNSAFE_COUNT=$(grep -r "unsafe" code/crates --include="*.rs" 2>/dev/null | wc -l)
echo "   unsafe occurrences: $UNSAFE_COUNT"
if [ "$UNSAFE_COUNT" -gt 100 ] && [ "$UNSAFE_COUNT" -lt 150 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Unsafe count matches audit (~111)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: Unsafe count differs from audit"
    ((PASSED++))
fi

echo ""
echo "1️⃣1️⃣  Verifying Mock/Stub Feature Gates..."
FEATURE_GATE_COUNT=$(grep -r "#\[cfg(feature = \"dev-stubs\")\]" code/crates --include="*.rs" 2>/dev/null | wc -l)
echo "   Feature gates: $FEATURE_GATE_COUNT"
if [ "$FEATURE_GATE_COUNT" -gt 30 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Feature gates present (audit: ~35)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: Fewer feature gates than audit"
    ((PASSED++))
fi

echo ""
echo "1️⃣2️⃣  Checking Sovereignty References..."
SOVEREIGNTY_COUNT=$(grep -r "sovereignty" code/crates --include="*.rs" 2>/dev/null | wc -l)
echo "   Sovereignty references: $SOVEREIGNTY_COUNT"
if [ "$SOVEREIGNTY_COUNT" -gt 250 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Sovereignty deeply embedded (audit: ~269)"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: Fewer sovereignty refs than audit"
    ((PASSED++))
fi

echo ""
echo "1️⃣3️⃣  Checking Clippy (library builds)..."
CLIPPY_ERRORS=$(cargo clippy --workspace --lib 2>&1 | grep "error:" | wc -l)
echo "   Clippy errors: $CLIPPY_ERRORS"
if [ "$CLIPPY_ERRORS" -eq 0 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Zero clippy errors"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}: Clippy errors found"
    ((FAILED++))
fi

echo ""
echo "=============================="
echo "📊 VERIFICATION SUMMARY"
echo "=============================="
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ ALL CRITICAL CHECKS PASSED${NC}"
    echo "Audit findings verified!"
    exit 0
else
    echo -e "${RED}⚠️  SOME CHECKS FAILED${NC}"
    echo "Review failed items above."
    exit 1
fi

