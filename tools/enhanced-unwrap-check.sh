#!/bin/bash
# Enhanced unwrap checker with detailed reporting
# Shows progress toward unwrap elimination goals

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║         🔍 ENHANCED UNWRAP/PANIC PATTERN CHECKER               ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

cd "$(dirname "$0")/.."

# Production code analysis (excluding tests)
echo "📊 PRODUCTION CODE ANALYSIS (excluding tests):"
echo "─────────────────────────────────────────────────────"

PROD_UNWRAP=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep "\.unwrap()" {} \; | wc -l)

PROD_EXPECT=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep "\.expect(" {} \; | wc -l)

PROD_PANIC=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep "panic!" {} \; | wc -l)

PROD_TODO=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep "todo!()" {} \; | wc -l)

PROD_UNIMPL=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep "unimplemented!()" {} \; | wc -l)

TOTAL=$((PROD_UNWRAP + PROD_EXPECT + PROD_PANIC + PROD_TODO + PROD_UNIMPL))

echo "   .unwrap() calls:      $PROD_UNWRAP"
echo "   .expect() calls:      $PROD_EXPECT"
echo "   panic!() calls:       $PROD_PANIC"
echo "   todo!() calls:        $PROD_TODO"
echo "   unimplemented!():     $PROD_UNIMPL"
echo "   ────────────────────────────"
echo "   TOTAL PATTERNS:       $TOTAL"
echo ""

# File-level analysis
echo "📁 FILES WITH PATTERNS:"
echo "─────────────────────────────────────────────────────"

UNWRAP_FILES=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep -l "\.unwrap()" {} \; | wc -l)

EXPECT_FILES=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep -l "\.expect(" {} \; | wc -l)

PANIC_FILES=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" \
  -exec grep -l "panic!" {} \; | wc -l)

echo "   Files with .unwrap():  $UNWRAP_FILES"
echo "   Files with .expect():  $EXPECT_FILES"
echo "   Files with panic!():   $PANIC_FILES"
echo ""

# Progress toward goals
echo "🎯 PROGRESS TOWARD GOALS:"
echo "─────────────────────────────────────────────────────"

# Goals
UNWRAP_TARGET=10
EXPECT_TARGET=5
PANIC_TARGET=0
TOTAL_TARGET=15

# Calculate progress
if [ $PROD_UNWRAP -le $UNWRAP_TARGET ]; then
    UNWRAP_STATUS="✅"
else
    UNWRAP_PROGRESS=$((100 - (PROD_UNWRAP - UNWRAP_TARGET) * 100 / PROD_UNWRAP))
    if [ $UNWRAP_PROGRESS -lt 0 ]; then UNWRAP_PROGRESS=0; fi
    UNWRAP_STATUS="⚠️  ${UNWRAP_PROGRESS}%"
fi

if [ $PROD_EXPECT -le $EXPECT_TARGET ]; then
    EXPECT_STATUS="✅"
else
    EXPECT_PROGRESS=$((100 - (PROD_EXPECT - EXPECT_TARGET) * 100 / PROD_EXPECT))
    if [ $EXPECT_PROGRESS -lt 0 ]; then EXPECT_PROGRESS=0; fi
    EXPECT_STATUS="⚠️  ${EXPECT_PROGRESS}%"
fi

if [ $PROD_PANIC -le $PANIC_TARGET ]; then
    PANIC_STATUS="✅"
else
    PANIC_STATUS="🔴 $PROD_PANIC remaining"
fi

echo "   Unwraps:  $PROD_UNWRAP / $UNWRAP_TARGET target  $UNWRAP_STATUS"
echo "   Expects:  $PROD_EXPECT / $EXPECT_TARGET target   $EXPECT_STATUS"
echo "   Panics:   $PROD_PANIC / $PANIC_TARGET target   $PANIC_STATUS"
echo "   Total:    $TOTAL / $TOTAL_TARGET target"
echo ""

# Top offending files
echo "🔥 TOP 10 FILES WITH MOST PATTERNS:"
echo "─────────────────────────────────────────────────────"

find code/crates/*/src -name "*.rs" -not -path "*/tests/*" -exec sh -c '
  count=$(grep -c "\.unwrap()\|\.expect(\|panic!(" "$1" 2>/dev/null || echo 0)
  if [ "$count" -gt 0 ]; then
    echo "$count|$1"
  fi
' sh {} \; | sort -rn -t'|' -k1 | head -10 | while IFS='|' read count file; do
  printf "   %3d patterns - %s\n" "$count" "$file"
done

echo ""

# Risk assessment
echo "🎯 RISK ASSESSMENT:"
echo "─────────────────────────────────────────────────────"

if [ $TOTAL -le $TOTAL_TARGET ]; then
    RISK="🟢 LOW"
    RISK_DESC="Excellent! Pattern usage within production limits."
elif [ $TOTAL -le 100 ]; then
    RISK="🟡 MEDIUM"
    RISK_DESC="Good progress, continue migration work."
elif [ $TOTAL -le 300 ]; then
    RISK="🟠 HIGH"
    RISK_DESC="Significant patterns remain. Prioritize elimination."
else
    RISK="🔴 CRITICAL"
    RISK_DESC="High panic risk. Urgent migration needed."
fi

echo "   Risk Level: $RISK"
echo "   Assessment: $RISK_DESC"
echo ""

# Recommendations
echo "💡 RECOMMENDATIONS:"
echo "─────────────────────────────────────────────────────"

if [ $TOTAL -gt $TOTAL_TARGET ]; then
    echo "   1. Run unwrap migrator analysis:"
    echo "      ./tools/unwrap-migrator/target/debug/unwrap-migrator --analyze code/crates/"
    echo ""
    echo "   2. Apply safe migrations (90%+ confidence):"
    echo "      ./tools/unwrap-migrator/target/debug/unwrap-migrator \\"
    echo "        --fix --nestgate-mode --advanced --confidence 90 code/crates/"
    echo ""
    echo "   3. Review and test changes:"
    echo "      cargo test --lib --all-features"
else
    echo "   ✅ Great work! Pattern usage is within acceptable limits."
    echo "   💡 Consider enforcing stricter linting to prevent regressions."
fi

echo ""
echo "════════════════════════════════════════════════════════════════"

# Exit code based on thresholds
if [ $PROD_UNWRAP -gt 50 ] || [ $PROD_EXPECT -gt 10 ] || [ $PROD_PANIC -gt 0 ]; then
    echo "⚠️  EXIT CODE 1: Pattern usage exceeds recommended thresholds"
    exit 1
else
    echo "✅ EXIT CODE 0: Pattern usage within acceptable limits"
    exit 0
fi

