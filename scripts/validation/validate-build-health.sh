#!/bin/bash
# validate-build-health.sh
# Validates overall build health and code quality

set -euo pipefail

echo "🔍 **BUILD HEALTH VALIDATION**"
echo "==========================================="

cd "$(dirname "$0")/../.."

ERRORS=0
WARNINGS=0

# File size compliance
echo ""
echo "📏 Checking file size compliance..."
large_files=$(find code/crates -name "*.rs" -path "*/src/*" -exec wc -l {} \; 2>/dev/null | \
  awk '$1 > 2000 {print}' | wc -l | tr -d ' \n' || echo "0")
if [ "$large_files" -eq 0 ]; then
    echo "   ✅ All files <2000 lines (PERFECT)"
else
    echo "   ❌ Found $large_files files >2000 lines"
    find code/crates -name "*.rs" -path "*/src/*" -exec wc -l {} \; 2>/dev/null | \
      awk '$1 > 2000 {print $2 " (" $1 " lines)"}' | head -5
    ERRORS=$((ERRORS + 1))
fi

# Tech debt markers
echo ""
echo "📝 Checking tech debt markers..."
debt_markers=$(grep -r "TODO\|FIXME\|XXX\|HACK" code/crates --include="*.rs" 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
echo "   Found $debt_markers tech debt markers"
if [ "$debt_markers" -lt 10 ]; then
    echo "   ✅ Minimal tech debt markers (EXCELLENT)"
elif [ "$debt_markers" -lt 50 ]; then
    echo "   ⚠️  Moderate tech debt markers (acceptable)"
    WARNINGS=$((WARNINGS + 1))
else
    echo "   ❌ High number of tech debt markers (needs attention)"
    ERRORS=$((ERRORS + 1))
fi

# Should compile cleanly
echo ""
echo "🔨 Running cargo check..."
if cargo check --workspace --quiet 2>&1 | tee /tmp/cargo-check.log | grep -q "error"; then
    echo "   ❌ Workspace has compilation errors"
    echo "   First error:"
    head -20 /tmp/cargo-check.log
    ERRORS=$((ERRORS + 1))
else
    echo "   ✅ Workspace compiles successfully"
fi

# Count total source files
echo ""
echo "📊 Codebase statistics..."
total_files=$(find code/crates -name "*.rs" -path "*/src/*" 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
total_lines=$(find code/crates -name "*.rs" -path "*/src/*" -exec wc -l {} \; 2>/dev/null | \
  awk '{sum += $1} END {print sum}' | tr -d ' \n' || echo "0")
echo "   Total source files: $total_files"
echo "   Total lines of code: $total_lines"
if [ "$total_files" -gt 0 ]; then
    avg_lines=$((total_lines / total_files))
    echo "   Average file size: $avg_lines lines"
fi

echo ""
echo "==========================================="
if [ "$ERRORS" -eq 0 ]; then
    if [ "$WARNINGS" -eq 0 ]; then
        echo "✅ **BUILD HEALTH: EXCELLENT**"
    else
        echo "✅ **BUILD HEALTH: GOOD** ($WARNINGS warnings)"
    fi
    exit 0
else
    echo "❌ **BUILD HEALTH: NEEDS ATTENTION** ($ERRORS errors, $WARNINGS warnings)"
    exit 1
fi 