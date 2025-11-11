#!/bin/bash
# Test all showcase demos to ensure they work

set -e

echo "🧪 =================================================="
echo "🧪  TESTING ALL NESTGATE DEMOS"
echo "🧪 =================================================="
echo ""

PASS=0
FAIL=0
TOTAL=0

test_demo() {
    local demo_path=$1
    local demo_name=$(basename $(dirname $demo_path))
    
    echo -n "Testing $demo_name... "
    TOTAL=$((TOTAL + 1))
    
    if timeout 120 $demo_path > /tmp/demo_test_$$.log 2>&1; then
        echo "✅ PASS"
        PASS=$((PASS + 1))
    else
        echo "❌ FAIL"
        FAIL=$((FAIL + 1))
        echo "   Error log: /tmp/demo_test_$$.log"
    fi
}

echo "🔍 Discovering demos..."
DEMOS=$(find demos -name "demo.sh" -type f | sort)
echo "Found $(echo "$DEMOS" | wc -l) demos"
echo ""

echo "🧪 Running tests..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

for demo in $DEMOS; do
    test_demo "$demo"
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 TEST RESULTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Total:  $TOTAL demos"
echo "Passed: $PASS demos"
echo "Failed: $FAIL demos"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "✅ ALL TESTS PASSED!"
    echo ""
    echo "🎊 All $TOTAL demos are working perfectly!"
    exit 0
else
    echo "⚠️  Some tests failed. Check logs above."
    exit 1
fi

