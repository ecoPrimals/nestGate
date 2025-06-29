#!/bin/bash
# NestGate Test Coverage Analysis Script
set -e

echo "🔍 NestGate Test Coverage Analysis"
echo "=================================="
echo

# Test execution summary
echo "📊 Test Execution Summary:"
echo "--------------------------"

TOTAL_TESTS=0
TOTAL_PASSED=0
TOTAL_FAILED=0
TOTAL_IGNORED=0

# Count source vs test files
SOURCE_FILES=$(find . -name "*.rs" -path "*/src/*" | wc -l)
TEST_FILES=$(find . -name "*.rs" -path "*/tests/*" | wc -l)
BROKEN_FILES=$(find . -name "*.rs.broken" | wc -l)

echo "Source files: $SOURCE_FILES"
echo "Active test files: $TEST_FILES"
echo "Broken test files: $BROKEN_FILES"
echo

# Run tests for working packages
echo "🧪 Package Test Results:"
echo "------------------------"

PACKAGES="nestgate-core nestgate-api nestgate-network nestgate-nas nestgate-mcp nestgate-automation nestgate-ai-models nestgate-middleware nestgate-fsmonitor nestgate-ui"
BIN_PACKAGES="nestgate-bin"

for package in $PACKAGES; do
    echo -n "Testing $package... "
    if timeout 60s cargo test --package $package --lib --quiet 2>/dev/null; then
        RESULT=$(cargo test --package $package --lib --quiet 2>&1 | grep "test result:" | tail -1)
        if [[ $RESULT =~ ([0-9]+)\ passed.*([0-9]+)\ failed.*([0-9]+)\ ignored ]]; then
            PASSED=${BASH_REMATCH[1]}
            FAILED=${BASH_REMATCH[2]}
            IGNORED=${BASH_REMATCH[3]}
            TOTAL_PASSED=$((TOTAL_PASSED + PASSED))
            TOTAL_FAILED=$((TOTAL_FAILED + FAILED))
            TOTAL_IGNORED=$((TOTAL_IGNORED + IGNORED))
            echo "✅ $PASSED passed, $FAILED failed, $IGNORED ignored"
        else
            echo "✅ Success (parsing failed)"
            TOTAL_PASSED=$((TOTAL_PASSED + 1))
        fi
    else
        echo "❌ Failed/Timeout"
        TOTAL_FAILED=$((TOTAL_FAILED + 1))
    fi
done

# Test binary packages separately 
echo
echo "🔨 Binary Package Tests:"
echo "------------------------"

for package in $BIN_PACKAGES; do
    echo -n "Testing $package... "
    if timeout 90s cargo test --package $package --quiet 2>/dev/null; then
        RESULT=$(cargo test --package $package --quiet 2>&1 | grep "test result:" | tail -1)
        if [[ $RESULT =~ ([0-9]+)\ passed.*([0-9]+)\ failed.*([0-9]+)\ ignored ]]; then
            PASSED=${BASH_REMATCH[1]}
            FAILED=${BASH_REMATCH[2]}
            IGNORED=${BASH_REMATCH[3]}
            TOTAL_PASSED=$((TOTAL_PASSED + PASSED))
            TOTAL_FAILED=$((TOTAL_FAILED + FAILED))
            TOTAL_IGNORED=$((TOTAL_IGNORED + IGNORED))
            echo "✅ $PASSED passed, $FAILED failed, $IGNORED ignored"
        else
            echo "✅ Success (parsing failed)"
            TOTAL_PASSED=$((TOTAL_PASSED + 1))
        fi
    else
        echo "❌ Failed/Timeout"
        TOTAL_FAILED=$((TOTAL_FAILED + 1))
    fi
done

TOTAL_TESTS=$((TOTAL_PASSED + TOTAL_FAILED + TOTAL_IGNORED))

echo
echo "📈 Overall Statistics:"
echo "---------------------"
echo "Total tests: $TOTAL_TESTS"
echo "Passed: $TOTAL_PASSED ($(((TOTAL_PASSED * 100) / (TOTAL_TESTS > 0 ? TOTAL_TESTS : 1)))%)"
echo "Failed: $TOTAL_FAILED"
echo "Ignored: $TOTAL_IGNORED"
echo

# Identify packages with low test counts
echo "🎯 Test Density Analysis:"
echo "-------------------------"

LOW_TEST_PACKAGES=""
HIGH_TEST_PACKAGES=""

for package in $PACKAGES; do
    if [ -d "code/crates/$package" ]; then
        SRC_COUNT=$(find "code/crates/$package/src" -name "*.rs" 2>/dev/null | wc -l)
        TEST_COUNT=$(find "code/crates/$package" -name "*test*.rs" -o -name "tests.rs" 2>/dev/null | wc -l)
        
        if [ "$SRC_COUNT" -gt 0 ]; then
            RATIO=$((TEST_COUNT * 100 / SRC_COUNT))
            
            if [ "$RATIO" -lt 20 ]; then
                LOW_TEST_PACKAGES="$LOW_TEST_PACKAGES $package"
                echo "⚠️  $package: $TEST_COUNT test files / $SRC_COUNT src files ($RATIO%)"
            else
                HIGH_TEST_PACKAGES="$HIGH_TEST_PACKAGES $package"
                echo "✅ $package: $TEST_COUNT test files / $SRC_COUNT src files ($RATIO%)"
            fi
        fi
    fi
done

echo
echo "🚀 Recommendations:"
echo "-------------------"

if [ -n "$LOW_TEST_PACKAGES" ]; then
    echo "Priority for additional tests:$LOW_TEST_PACKAGES"
else
    echo "All packages have good test coverage ratios!"
fi

echo
echo "🎖️  Success Rate: $(((TOTAL_PASSED * 100) / (TOTAL_TESTS > 0 ? TOTAL_TESTS : 1)))%"

if [ "$TOTAL_FAILED" -eq 0 ]; then
    echo "✨ Perfect test execution - Zero failures!"
else
    echo "⚠️  $TOTAL_FAILED failing tests need attention"
fi

echo
echo "Done! 🎉" 