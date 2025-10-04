#!/bin/bash
# run-all-validations.sh
# Runs all validation scripts and provides comprehensive report

set -euo pipefail

echo "🎯 **NESTGATE UNIFICATION VALIDATION SUITE**"
echo "============================================="
echo ""

cd "$(dirname "$0")/../.."

VALIDATION_DIR="scripts/validation"

TOTAL_PASSED=0
TOTAL_FAILED=0

# Make scripts executable
chmod +x "$VALIDATION_DIR"/*.sh 2>/dev/null || true

# Run each validation script
VALIDATIONS=(
    "validate-build-health.sh:Build Health"
    "validate-config-unification.sh:Configuration Unification"
    "validate-error-unification.sh:Error System Unification"
    "validate-deprecated-removal.sh:Deprecated Code Removal"
)

echo "Running validation suite..."
echo ""

for validation in "${VALIDATIONS[@]}"; do
    script="${validation%%:*}"
    name="${validation##*:}"
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📋 Validating: $name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    if [ -f "$VALIDATION_DIR/$script" ]; then
        if bash "$VALIDATION_DIR/$script"; then
            TOTAL_PASSED=$((TOTAL_PASSED + 1))
        else
            TOTAL_FAILED=$((TOTAL_FAILED + 1))
        fi
    else
        echo "⚠️  Script not found: $script"
        TOTAL_FAILED=$((TOTAL_FAILED + 1))
    fi
    
    echo ""
done

# Final summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 **VALIDATION SUMMARY**"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "   ✅ Passed: $TOTAL_PASSED"
echo "   ❌ Failed: $TOTAL_FAILED"
echo ""

if [ "$TOTAL_FAILED" -eq 0 ]; then
    echo "🎉 **ALL VALIDATIONS PASSED**"
    echo ""
    echo "Your codebase meets all unification criteria!"
    exit 0
else
    echo "⚠️  **SOME VALIDATIONS FAILED**"
    echo ""
    echo "Review the failures above and address issues."
    echo "Note: Some warnings are expected during migration phases."
    exit 1
fi 