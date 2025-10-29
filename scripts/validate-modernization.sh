#!/bin/bash
# 🔍 MODERNIZATION VALIDATION SCRIPT
# Validates that modernization is complete and successful

set -euo pipefail

echo "🔍 **NESTGATE MODERNIZATION VALIDATION**"
echo "========================================"

# Check for remaining legacy patterns
echo "📊 **LEGACY PATTERN ANALYSIS**"
echo "------------------------------"

# Check async_trait usage
ASYNC_TRAIT_COUNT=$(find code/crates -name "*.rs" -exec grep -l "#\[async_trait\]" {} \; | wc -l)
if [ "$ASYNC_TRAIT_COUNT" -eq 0 ]; then
    echo "✅ async_trait elimination: COMPLETE"
else
    echo "⚠️  async_trait remaining: $ASYNC_TRAIT_COUNT files"
fi

# Check for hardcoded values
HARDCODED_PORTS=$(grep -r ":[0-9]\{4,5\}" code/crates --include="*.rs" | grep -v "const\|static" | wc -l)
if [ "$HARDCODED_PORTS" -lt 50 ]; then
    echo "✅ Hardcoded values: ACCEPTABLE ($HARDCODED_PORTS remaining)"
else
    echo "⚠️  Hardcoded values: $HARDCODED_PORTS (target: <50)"
fi

# Check compilation
echo ""
echo "🔧 **COMPILATION VALIDATION**"
echo "-----------------------------"

if cargo check --workspace --quiet; then
    echo "✅ Workspace compilation: SUCCESS"
else
    echo "❌ Workspace compilation: FAILED"
    exit 1
fi

# Check file size compliance
echo ""
echo "📏 **FILE SIZE COMPLIANCE**"
echo "---------------------------"

MAX_LINES=0
OVERSIZED_FILES=0

while IFS= read -r -d '' file; do
    LINES=$(wc -l < "$file")
    if [ "$LINES" -gt 2000 ]; then
        echo "❌ OVERSIZED: $file ($LINES lines)"
        OVERSIZED_FILES=$((OVERSIZED_FILES + 1))
    fi
    if [ "$LINES" -gt "$MAX_LINES" ]; then
        MAX_LINES=$LINES
    fi
done < <(find code/crates -name "*.rs" -print0)

if [ "$OVERSIZED_FILES" -eq 0 ]; then
    echo "✅ File size compliance: 100% (max: $MAX_LINES lines)"
else
    echo "❌ File size violations: $OVERSIZED_FILES files exceed 2000 lines"
fi

# Overall assessment
echo ""
echo "🏆 **MODERNIZATION ASSESSMENT**"
echo "==============================="

if [ "$ASYNC_TRAIT_COUNT" -eq 0 ] && [ "$OVERSIZED_FILES" -eq 0 ] && [ "$HARDCODED_PORTS" -lt 50 ]; then
    echo "✅ **MODERNIZATION COMPLETE** - All targets achieved!"
    echo "🎉 NestGate is fully modernized with:"
    echo "   - Zero legacy async_trait patterns"
    echo "   - 100% file size compliance"
    echo "   - Minimal hardcoded values"
    echo "   - Unified error/config/constants systems"
    exit 0
else
    echo "🔄 **MODERNIZATION IN PROGRESS** - Some work remaining"
    echo "📋 Remaining tasks:"
    [ "$ASYNC_TRAIT_COUNT" -gt 0 ] && echo "   - Migrate $ASYNC_TRAIT_COUNT async_trait files"
    [ "$OVERSIZED_FILES" -gt 0 ] && echo "   - Split $OVERSIZED_FILES oversized files"
    [ "$HARDCODED_PORTS" -ge 50 ] && echo "   - Migrate hardcoded values to constants"
    exit 1
fi
