#!/bin/bash
# Validate ModuleError migration results

echo "🔍 VALIDATING MODULEERROR MIGRATION"
echo "=================================="

ERRORS=0

# Check for remaining non-deprecated ModuleError
REMAINING_MODULE_ERRORS=$(find code/crates/nestgate-core -name "*.rs" -exec grep -l "pub enum ModuleError" {} \; | wc -l)
if [ "$REMAINING_MODULE_ERRORS" -gt 0 ]; then
    echo "❌ Found $REMAINING_MODULE_ERRORS files with non-deprecated ModuleError"
    ERRORS=$((ERRORS + 1))
else
    echo "✅ No non-deprecated ModuleError found"
fi

# Check for NestGateUnifiedError imports
UNIFIED_IMPORTS=$(find code/crates/nestgate-core -name "*.rs" -exec grep -l "use.*NestGateUnifiedError" {} \; | wc -l)
echo "✅ Found $UNIFIED_IMPORTS files with NestGateUnifiedError imports"

# Check compilation
echo "🔧 Testing compilation..."
if cargo check --quiet 2>/dev/null; then
    echo "✅ Compilation successful"
else
    echo "❌ Compilation failed - migration needs adjustment"
    ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -eq 0 ]; then
    echo "✅ MIGRATION VALIDATION SUCCESSFUL"
    exit 0
else
    echo "❌ MIGRATION VALIDATION FAILED ($ERRORS errors)"
    exit 1
fi
