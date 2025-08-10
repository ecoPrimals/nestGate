#!/bin/bash
# File Size Compliance Check - Enforce 2000 line maximum

echo "🔍 NESTGATE FILE SIZE COMPLIANCE CHECK"
echo "======================================"

VIOLATIONS=0
TOTAL_FILES=0

echo "📊 Checking all .rs files in code/ directory..."

while IFS= read -r -d '' file; do
    TOTAL_FILES=$((TOTAL_FILES + 1))
    LINES=$(wc -l < "$file")
    
    if [ "$LINES" -gt 2000 ]; then
        echo "❌ VIOLATION: $file ($LINES lines)"
        VIOLATIONS=$((VIOLATIONS + 1))
    elif [ "$LINES" -gt 1800 ]; then
        echo "⚠️  WARNING: $file ($LINES lines) - approaching limit"
    fi
done < <(find ./code -name "*.rs" -print0)

echo ""
echo "📈 SUMMARY:"
echo "  Total files checked: $TOTAL_FILES"
echo "  Files over 2000 lines: $VIOLATIONS"

if [ "$VIOLATIONS" -eq 0 ]; then 
    echo "✅ ALL FILES COMPLIANT - No files exceed 2000 lines"
    exit 0
else
    echo "❌ $VIOLATIONS FILES NEED SPLITTING"
    exit 1
fi 