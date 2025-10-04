#!/bin/bash
# validate-error-unification.sh
# Validates that error system unification is complete

set -euo pipefail

echo "🔍 **ERROR SYSTEM UNIFICATION VALIDATION**"
echo "==========================================="

cd "$(dirname "$0")/../.."

ERRORS=0

# Should find NO LegacyModuleError
echo ""
echo "❌ Checking for LegacyModuleError..."
legacy_count=$(rg "pub enum LegacyModuleError" --type rust code/crates/ 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
if [ "$legacy_count" -eq 0 ]; then
    echo "   ✅ No LegacyModuleError instances found"
else
    echo "   ❌ Found $legacy_count LegacyModuleError instances"
    echo "   Locations (first 10):"
    rg "pub enum LegacyModuleError" --type rust -l code/crates/ 2>/dev/null | head -10
    ERRORS=$((ERRORS + 1))
fi

# Should use NestGateUnifiedError everywhere
echo ""
echo "✅ Checking NestGateUnifiedError usage..."
unified_usage=$(rg "use.*NestGateUnifiedError|use.*NestGateError" --type rust code/crates/ 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
echo "   Found $unified_usage uses of NestGateUnifiedError/NestGateError"
if [ "$unified_usage" -gt 50 ]; then
    echo "   ✅ Good adoption of unified error system"
else
    echo "   ⚠️  Low usage of unified error system ($unified_usage uses)"
fi

# Should find NO domain error enums in production code (excluding test/tool/Details/Context)
echo ""
echo "❌ Checking for domain-specific error enums..."
domain_errors=$(rg "pub enum.*Error" --type rust code/crates/ 2>/dev/null | \
  grep -v "test\|tool\|Details\|Context\|Severity\|Category\|NestGate" | wc -l | tr -d ' \n' || echo "0")
if [ "$domain_errors" -lt 20 ]; then
    echo "   ✅ Minimal domain-specific errors ($domain_errors)"
else
    echo "   ⚠️  Found $domain_errors domain-specific error enums (review recommended)"
    echo "   Note: Some may be legitimate (test doubles, tools, etc.)"
fi

# Check for error migration helpers
echo ""
echo "❌ Checking for error migration helpers..."
if [ -d "code/crates/nestgate-core/src/error/migration_helpers" ]; then
    helper_count=$(find code/crates/nestgate-core/src/error/migration_helpers -name "*.rs" 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
    if [ "$helper_count" -gt 0 ]; then
        echo "   ⚠️  Found $helper_count error migration helper files (can be removed after migration)"
    fi
else
    echo "   ✅ Error migration helpers directory already removed"
fi

echo ""
echo "==========================================="
if [ "$ERRORS" -eq 0 ]; then
    echo "✅ **ERROR SYSTEM UNIFICATION: PASSED**"
    exit 0
else
    echo "❌ **ERROR SYSTEM UNIFICATION: FAILED** ($ERRORS issues)"
    exit 1
fi 