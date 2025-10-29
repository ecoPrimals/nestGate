#!/bin/bash
# Configuration Unification Validation Script

set -euo pipefail

PROJECT_ROOT="$1"
cd "$PROJECT_ROOT"

echo "🔍 CONFIGURATION UNIFICATION VALIDATION"
echo "========================================"
echo ""

PASSED=0
FAILED=0

# Test 1: Only canonical_master should have NetworkConfig
echo "Test 1: NetworkConfig consolidation..."
NON_CANONICAL=$(rg "pub struct.*NetworkConfig" --type rust code/crates/ | grep -v canonical_master | wc -l || true)
if [ "$NON_CANONICAL" -eq 0 ]; then
    echo "  ✅ PASS: NetworkConfig only in canonical_master"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $NON_CANONICAL NetworkConfig definitions outside canonical_master"
    FAILED=$((FAILED + 1))
fi

# Test 2: Only canonical_master should have StorageConfig
echo "Test 2: StorageConfig consolidation..."
NON_CANONICAL=$(rg "pub struct.*StorageConfig" --type rust code/crates/ | grep -v canonical_master | wc -l || true)
if [ "$NON_CANONICAL" -eq 0 ]; then
    echo "  ✅ PASS: StorageConfig only in canonical_master"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $NON_CANONICAL StorageConfig definitions outside canonical_master"
    FAILED=$((FAILED + 1))
fi

# Test 3: No usage of deprecated CanonicalConfig
echo "Test 3: No deprecated CanonicalConfig usage..."
DEPRECATED_USAGE=$(rg "use.*canonical::types::CanonicalConfig" --type rust code/crates/ | wc -l || true)
if [ "$DEPRECATED_USAGE" -eq 0 ]; then
    echo "  ✅ PASS: No usage of deprecated CanonicalConfig"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $DEPRECATED_USAGE uses of deprecated CanonicalConfig"
    FAILED=$((FAILED + 1))
fi

# Test 4: No usage of deprecated StandardDomainConfig
echo "Test 4: No deprecated StandardDomainConfig usage..."
DEPRECATED_USAGE=$(rg "use.*StandardDomainConfig" --type rust code/crates/ | wc -l || true)
if [ "$DEPRECATED_USAGE" -eq 0 ]; then
    echo "  ✅ PASS: No usage of deprecated StandardDomainConfig"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $DEPRECATED_USAGE uses of deprecated StandardDomainConfig"
    FAILED=$((FAILED + 1))
fi

# Test 5: Build should succeed
echo "Test 5: Clean build..."
if cargo check --workspace 2>&1 | grep -q "error"; then
    echo "  ❌ FAIL: Build has errors"
    FAILED=$((FAILED + 1))
else
    echo "  ✅ PASS: Build succeeds"
    PASSED=$((PASSED + 1))
fi

# Summary
echo ""
echo "VALIDATION SUMMARY"
echo "=================="
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo ""

if [ "$FAILED" -eq 0 ]; then
    echo "🎉 ALL TESTS PASSED! Configuration unification complete!"
    exit 0
else
    echo "❌ Some tests failed. Continue working on unification."
    exit 1
fi
