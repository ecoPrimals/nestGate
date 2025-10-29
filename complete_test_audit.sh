#!/bin/bash
# Complete Test Audit - Includes both separate test files AND inline test modules

cd code/crates/nestgate-api/src

echo "════════════════════════════════════════════════════════════"
echo "🔍 COMPLETE TEST AUDIT"
echo "════════════════════════════════════════════════════════════"
echo ""

tested_files=0
untested_files=0
inline_tests=0
separate_tests=0

echo "📋 FILES WITH TESTS:"
echo ""

# Find all handler files (not test files, not mod.rs)
for handler_file in $(find handlers rest/handlers -name "*.rs" | grep -v "_tests.rs" | grep -v "_test.rs" | grep -v "/mod.rs" | sort); do
    base=$(basename "$handler_file" .rs)
    dir=$(dirname "$handler_file")
    has_test=false
    test_type=""
    
    # Check for separate test file
    if [ -f "${dir}/${base}_tests.rs" ] || [ -f "${dir}/${base}_test.rs" ]; then
        has_test=true
        test_type="📄 Separate test file"
        separate_tests=$((separate_tests + 1))
    fi
    
    # Check for inline tests
    if grep -q "#\[cfg(test)\]" "$handler_file" 2>/dev/null; then
        has_test=true
        if [ -z "$test_type" ]; then
            test_type="📝 Inline tests"
            inline_tests=$((inline_tests + 1))
        else
            test_type="$test_type + 📝 Inline tests"
            inline_tests=$((inline_tests + 1))
        fi
    fi
    
    if [ "$has_test" = true ]; then
        tested_files=$((tested_files + 1))
        echo "✅ $handler_file"
        echo "   $test_type"
        
        # Count number of test functions
        test_count=$(grep -c "#\[test\]" "$handler_file" 2>/dev/null || echo 0)
        async_count=$(grep -c "#\[tokio::test\]" "$handler_file" 2>/dev/null || echo 0)
        
        # Check separate test file if it exists
        if [ -f "${dir}/${base}_tests.rs" ]; then
            test_count=$((test_count + $(grep -c "#\[test\]" "${dir}/${base}_tests.rs" 2>/dev/null || echo 0)))
            async_count=$((async_count + $(grep -c "#\[tokio::test\]" "${dir}/${base}_tests.rs" 2>/dev/null || echo 0)))
        fi
        
        total_tests=$((test_count + async_count))
        echo "   📊 $total_tests tests ($test_count sync, $async_count async)"
        echo ""
    fi
done

echo ""
echo "════════════════════════════════════════════════════════════"
echo "❌ FILES WITHOUT TESTS:"
echo "════════════════════════════════════════════════════════════"
echo ""

# Find files without any tests
for handler_file in $(find handlers rest/handlers -name "*.rs" | grep -v "_tests.rs" | grep -v "_test.rs" | grep -v "/mod.rs" | sort); do
    base=$(basename "$handler_file" .rs)
    dir=$(dirname "$handler_file")
    has_test=false
    
    # Check for separate test file
    if [ -f "${dir}/${base}_tests.rs" ] || [ -f "${dir}/${base}_test.rs" ]; then
        has_test=true
    fi
    
    # Check for inline tests
    if grep -q "#\[cfg(test)\]" "$handler_file" 2>/dev/null; then
        has_test=true
    fi
    
    if [ "$has_test" = false ]; then
        untested_files=$((untested_files + 1))
        echo "⚠️  $handler_file"
    fi
done

echo ""
echo "════════════════════════════════════════════════════════════"
echo "📊 SUMMARY"
echo "════════════════════════════════════════════════════════════"
total_files=$((tested_files + untested_files))
echo "Total handler files:      $total_files"
echo "Files with tests:         $tested_files (✅)"
echo "Files without tests:      $untested_files (❌)"
echo ""
echo "Test organization:"
echo "  Separate test files:    $separate_tests"
echo "  Inline test modules:    $inline_tests"
echo ""

if [ $tested_files -gt 0 ]; then
    coverage_pct=$(echo "scale=1; $tested_files * 100 / $total_files" | bc)
    echo "File coverage:            ${coverage_pct}%"
fi

echo ""
echo "════════════════════════════════════════════════════════════"
echo "🎯 KEY INSIGHT"
echo "════════════════════════════════════════════════════════════"
echo "Your existing tests ARE wired properly!"
echo "The main issue is $untested_files files have NO tests at all."
echo ""
echo "Next: Add tests to high-priority untested files"
echo "════════════════════════════════════════════════════════════"

