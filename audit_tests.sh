#!/bin/bash
# Test File Audit Script
# Identifies which test files are properly wired to production code

cd code/crates/nestgate-api/src

echo "════════════════════════════════════════════════════════════"
echo "🔍 TEST FILE AUDIT - Wiring Status"
echo "════════════════════════════════════════════════════════════"
echo ""

total_tests=0
wired_tests=0
disconnected_tests=0
async_tests=0

# Find all test files
for test_file in $(find . -name "*_tests.rs" -o -name "*_test.rs" | sort); do
    total_tests=$((total_tests + 1))
    
    file_name=$(basename "$test_file")
    echo "📄 $test_file"
    
    # Check if it imports from production
    has_imports=false
    if grep -q "use super::" "$test_file" 2>/dev/null || grep -q "use crate::" "$test_file" 2>/dev/null; then
        echo "   ✅ Has production imports"
        has_imports=true
    else
        echo "   ⚠️  No production imports"
    fi
    
    # Check if it has async tests (likely calls async production functions)
    has_async=false
    if grep -q "#\[tokio::test\]" "$test_file" 2>/dev/null; then
        echo "   ✅ Has async integration tests"
        has_async=true
        async_tests=$((async_tests + 1))
    else
        echo "   ⚠️  Only sync tests (might be type-only)"
    fi
    
    # Count actual test functions
    test_count=$(grep -c "#\[test\]" "$test_file" 2>/dev/null || echo 0)
    async_count=$(grep -c "#\[tokio::test\]" "$test_file" 2>/dev/null || echo 0)
    total_count=$((test_count + async_count))
    echo "   📊 Test count: $total_count ($test_count sync, $async_count async)"
    
    # Determine if wired or disconnected
    if [ "$has_imports" = true ] && [ "$has_async" = true ]; then
        echo "   🟢 STATUS: WELL WIRED"
        wired_tests=$((wired_tests + 1))
    elif [ "$has_imports" = true ]; then
        echo "   🟡 STATUS: PARTIALLY WIRED"
    else
        echo "   🔴 STATUS: DISCONNECTED (type tests only)"
        disconnected_tests=$((disconnected_tests + 1))
    fi
    
    echo ""
done

echo "════════════════════════════════════════════════════════════"
echo "📊 SUMMARY"
echo "════════════════════════════════════════════════════════════"
echo "Total test files: $total_tests"
echo "Well wired:       $wired_tests (🟢)"
echo "Disconnected:     $disconnected_tests (🔴)"
echo "Have async tests: $async_tests"
echo ""

echo "════════════════════════════════════════════════════════════"
echo "🎯 RECOMMENDATIONS"
echo "════════════════════════════════════════════════════════════"

if [ $disconnected_tests -gt 0 ]; then
    echo "1. Review disconnected test files (🔴)"
    echo "2. Add 'use super::*' imports"
    echo "3. Add async integration tests that call production functions"
    echo "4. Re-run this script to verify"
else
    echo "✅ All test files appear to be wired!"
    echo "   Run 'cargo tarpaulin' to verify coverage"
fi

echo ""
echo "════════════════════════════════════════════════════════════"
echo "📋 FILES WITHOUT TESTS"
echo "════════════════════════════════════════════════════════════"

# Find .rs files without corresponding _tests.rs
for handler_file in $(find handlers rest/handlers -name "*.rs" | grep -v "_tests.rs" | grep -v "_test.rs" | grep -v "/mod.rs" | sort); do
    base=$(basename "$handler_file" .rs)
    dir=$(dirname "$handler_file")
    
    # Check if corresponding test file exists
    if [ ! -f "${dir}/${base}_tests.rs" ] && [ ! -f "${dir}/${base}_test.rs" ]; then
        echo "⚠️  Missing tests: $handler_file"
    fi
done

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Next: Run 'cargo tarpaulin --out Html' to see actual coverage"
echo "════════════════════════════════════════════════════════════"

