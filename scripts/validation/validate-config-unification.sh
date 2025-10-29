#!/bin/bash
# validate-config-unification.sh
# Validates that configuration unification is complete

set -euo pipefail

echo "🔍 **CONFIGURATION UNIFICATION VALIDATION**"
echo "==========================================="

cd "$(dirname "$0")/../.."

ERRORS=0

# Should find ONLY canonical_master configs
echo ""
echo "✅ Checking for canonical_master configs..."
canonical_count=$(rg "pub struct.*NetworkConfig" --type rust \
  code/crates/nestgate-core/src/config/canonical_master 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
echo "   Found $canonical_count canonical NetworkConfig definitions"

# Should find NO other NetworkConfig definitions
echo ""
echo "❌ Checking for duplicate NetworkConfig definitions..."
duplicate_count=$(rg "pub struct.*NetworkConfig" --type rust code/crates/ 2>/dev/null | \
  grep -v canonical_master | wc -l | tr -d ' \n' || echo "0")
if [ "$duplicate_count" -eq 0 ]; then
    echo "   ✅ No duplicate NetworkConfig definitions found"
else
    echo "   ❌ Found $duplicate_count duplicate NetworkConfig definitions"
    echo "   Locations:"
    rg "pub struct.*NetworkConfig" --type rust code/crates/ 2>/dev/null | grep -v canonical_master | head -10
    ERRORS=$((ERRORS + 1))
fi

# Should find NO usage of deprecated configs
echo ""
echo "❌ Checking for usage of deprecated configs..."
deprecated_usage=$(rg "use.*canonical::types::CanonicalConfig" --type rust code/crates/ 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
if [ "$deprecated_usage" -eq 0 ]; then
    echo "   ✅ No usage of deprecated CanonicalConfig found"
else
    echo "   ❌ Found $deprecated_usage usages of deprecated configs"
    ERRORS=$((ERRORS + 1))
fi

# Check for StandardDomainConfig usage
standard_usage=$(rg "use.*StandardDomainConfig" --type rust code/crates/ 2>/dev/null | wc -l | tr -d ' \n' || echo "0")
if [ "$standard_usage" -eq 0 ]; then
    echo "   ✅ No usage of StandardDomainConfig found"
else
    echo "   ❌ Found $standard_usage usages of StandardDomainConfig"
    ERRORS=$((ERRORS + 1))
fi

echo ""
echo "==========================================="
if [ "$ERRORS" -eq 0 ]; then
    echo "✅ **CONFIGURATION UNIFICATION: PASSED**"
    exit 0
else
    echo "❌ **CONFIGURATION UNIFICATION: FAILED** ($ERRORS issues)"
    exit 1
fi 