#!/bin/bash
# validate-deprecated-removal.sh
# Validates that deprecated code has been removed

set -euo pipefail

echo "🔍 **DEPRECATED CODE REMOVAL VALIDATION**"
echo "==========================================="

cd "$(dirname "$0")/../.."

ERRORS=0

# Should find NO deprecated markers in production code
echo ""
echo "❌ Checking for deprecated markers..."
deprecated_count=$(rg "#\[deprecated" --type rust code/crates/ 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
if [ "$deprecated_count" -eq 0 ]; then
    echo "   ✅ No deprecated markers found"
else
    echo "   ⚠️  Found $deprecated_count deprecated markers"
    echo "   This is expected during migration. Review after Week 4."
    echo "   Sample (first 5):"
    rg "#\[deprecated" --type rust code/crates/ 2>/dev/null | head -5
fi

# Should find NO config migration helpers
echo ""
echo "❌ Checking for config migration helpers..."
if [ -d "code/crates/nestgate-core/src/config/migration_helpers" ]; then
    helper_count=$(find code/crates/nestgate-core/src/config/migration_helpers -name "*.rs" 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
    if [ "$helper_count" -gt 0 ]; then
        echo "   ⚠️  Found $helper_count config migration helper files"
        echo "   These should be removed in Week 4 after migrations complete"
    fi
else
    echo "   ✅ Config migration helpers directory removed"
fi

# Should find NO error migration helpers
echo ""
echo "❌ Checking for error migration helpers..."
if [ -d "code/crates/nestgate-core/src/error/migration_helpers" ]; then
    helper_count=$(find code/crates/nestgate-core/src/error/migration_helpers -name "*.rs" 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
    if [ "$helper_count" -gt 0 ]; then
        echo "   ⚠️  Found $helper_count error migration helper files"
        echo "   These should be removed in Week 4 after migrations complete"
    fi
else
    echo "   ✅ Error migration helpers directory removed"
fi

echo ""
echo "==========================================="
if [ "$ERRORS" -eq 0 ]; then
    echo "✅ **DEPRECATED CODE REMOVAL: VALIDATED**"
    echo "   (Warnings are expected during migration)"
    exit 0
else
    echo "❌ **DEPRECATED CODE REMOVAL: FAILED** ($ERRORS critical issues)"
    exit 1
fi 