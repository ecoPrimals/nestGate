#!/bin/bash
# Deprecated Code Cleanup - Systematic Technical Debt Elimination

echo "🧹 NESTGATE DEPRECATED CODE CLEANUP"
echo "==================================="

DEPRECATED_ITEMS=0
DEPRECATED_ALLOWS=0

echo "📊 Scanning for deprecated items..."

# Count deprecated attributes
DEPRECATED_ATTRS=$(grep -r "#\[deprecated" code/ --include="*.rs" | wc -l)
echo "🏷️  Deprecated attributes found: $DEPRECATED_ATTRS"

# Count deprecated allows  
DEPRECATED_ALLOWS=$(grep -r "#\[allow(deprecated)\]" code/ --include="*.rs" | wc -l)
echo "⚠️  Allow deprecated found: $DEPRECATED_ALLOWS"

# Find specific deprecated patterns
echo ""
echo "🔍 DEPRECATED PATTERNS BY CATEGORY:"

echo "📋 Config Structs:"
grep -r "DEPRECATED.*Use Unified.*Config" code/ --include="*.rs" | head -5
echo "   ... (showing first 5, total found: $(grep -r "DEPRECATED.*Use Unified.*Config" code/ --include="*.rs" | wc -l))"

echo ""
echo "🔤 Enum Types:"
grep -r "DEPRECATED.*Use.*Unified.*from.*unified_enums" code/ --include="*.rs" | head -5
echo "   ... (showing first 5, total found: $(grep -r "DEPRECATED.*Use.*Unified.*from.*unified_enums" code/ --include="*.rs" | wc -l))"

echo ""
echo "🏗️  Result Types:"
grep -r "Result<T>" code/ --include="*.rs" | grep -v "std::result::Result\|NestGateError" | head -5
echo "   ... (showing first 5 non-unified Result types)"

echo ""
echo "📈 CLEANUP SUMMARY:"
TOTAL_DEBT=$((DEPRECATED_ATTRS + DEPRECATED_ALLOWS))
echo "  Total deprecated debt items: $TOTAL_DEBT"

if [ "$TOTAL_DEBT" -eq 0 ]; then
    echo "✅ CLEAN CODEBASE - No deprecated items found"
    exit 0
else
    echo "🎯 CLEANUP OPPORTUNITY - $TOTAL_DEBT items ready for systematic removal"
    echo ""
    echo "💡 RECOMMENDED ACTIONS:"
    echo "  1. Remove deprecated structs/enums with unified replacements"
    echo "  2. Update #[allow(deprecated)] usage to use unified types"
    echo "  3. Consolidate duplicate configuration patterns"
    echo "  4. Migrate remaining Result<T> aliases to NestGateError"
    exit 1
fi 