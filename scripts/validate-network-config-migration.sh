#!/bin/bash
# NetworkConfig Migration Validation Script
# Validates the consolidation of NetworkConfig variants to canonical

set -euo pipefail

echo "╔══════════════════════════════════════════════════════╗"
echo "║  NetworkConfig Migration Validation                 ║"
echo "╚══════════════════════════════════════════════════════╝"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASS=0
FAIL=0
WARN=0

echo "📊 Analyzing NetworkConfig usage..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check 1: Old StandardDomainConfig pattern
echo "🔍 Check 1: Old StandardDomainConfig<NetworkExtensions> pattern"
OLD_IMPORTS=$(rg "StandardDomainConfig<NetworkExtensions>" --type rust code/crates 2>/dev/null | wc -l | xargs || echo "0")
if [ "$OLD_IMPORTS" -eq 0 ]; then
    echo -e "  ${GREEN}✅ PASS${NC}: No old StandardDomainConfig patterns found"
    ((PASS++))
else
    echo -e "  ${RED}❌ FAIL${NC}: Found $OLD_IMPORTS old StandardDomainConfig patterns"
    echo "  Files with old patterns:"
    rg "StandardDomainConfig<NetworkExtensions>" --type rust code/crates -l 2>/dev/null | head -5 | sed 's/^/    - /'
    ((FAIL++))
fi
echo ""

# Check 2: Canonical imports
echo "🔍 Check 2: Canonical NetworkConfig imports"
CANONICAL_IMPORTS=$(rg "CanonicalNetworkConfig" --type rust code/crates 2>/dev/null | wc -l | xargs || echo "0")
if [ "$CANONICAL_IMPORTS" -gt 10 ]; then
    echo -e "  ${GREEN}✅ PASS${NC}: Found $CANONICAL_IMPORTS canonical imports (good adoption)"
    ((PASS++))
elif [ "$CANONICAL_IMPORTS" -gt 0 ]; then
    echo -e "  ${YELLOW}⚠️  WARN${NC}: Found $CANONICAL_IMPORTS canonical imports (low adoption)"
    ((WARN++))
else
    echo -e "  ${RED}❌ FAIL${NC}: No canonical imports found"
    ((FAIL++))
fi
echo ""

# Check 3: Deprecated unified_config_consolidation usages
echo "🔍 Check 3: Deprecated unified_config_consolidation usages"
DEPRECATED=$(rg "unified_config_consolidation" --type rust code/crates 2>/dev/null | wc -l | xargs || echo "0")
if [ "$DEPRECATED" -eq 0 ]; then
    echo -e "  ${GREEN}✅ PASS${NC}: No deprecated unified_config_consolidation usages"
    ((PASS++))
else
    echo -e "  ${RED}❌ FAIL${NC}: Found $DEPRECATED deprecated pattern usages"
    echo "  Files still using deprecated patterns:"
    rg "unified_config_consolidation" --type rust code/crates -l 2>/dev/null | head -5 | sed 's/^/    - /'
    ((FAIL++))
fi
echo ""

# Check 4: Multiple NetworkConfig definitions
echo "🔍 Check 4: Multiple NetworkConfig struct definitions"
NETWORK_CONFIG_DEFS=$(rg "pub struct.*NetworkConfig\s*\{" --type rust code/crates 2>/dev/null | wc -l | xargs || echo "0")
EXPECTED_DEFS=1  # Only canonical should exist
if [ "$NETWORK_CONFIG_DEFS" -le "$EXPECTED_DEFS" ]; then
    echo -e "  ${GREEN}✅ PASS${NC}: Found $NETWORK_CONFIG_DEFS NetworkConfig definitions (target: $EXPECTED_DEFS)"
    ((PASS++))
else
    echo -e "  ${RED}❌ FAIL${NC}: Found $NETWORK_CONFIG_DEFS NetworkConfig definitions (expected: $EXPECTED_DEFS)"
    echo "  Multiple definitions found in:"
    rg "pub struct.*NetworkConfig" --type rust code/crates -l 2>/dev/null | head -5 | sed 's/^/    - /'
    ((FAIL++))
fi
echo ""

# Check 5: NetworkConfig type aliases
echo "🔍 Check 5: NetworkConfig type aliases pointing to canonical"
TYPE_ALIASES=$(rg "pub type NetworkConfig\s*=" --type rust code/crates 2>/dev/null | wc -l | xargs || echo "0")
if [ "$TYPE_ALIASES" -le 3 ]; then
    echo -e "  ${GREEN}✅ PASS${NC}: Found $TYPE_ALIASES NetworkConfig type aliases (reasonable)"
    ((PASS++))
elif [ "$TYPE_ALIASES" -le 10 ]; then
    echo -e "  ${YELLOW}⚠️  WARN${NC}: Found $TYPE_ALIASES NetworkConfig type aliases (could be consolidated)"
    ((WARN++))
else
    echo -e "  ${RED}❌ FAIL${NC}: Found $TYPE_ALIASES NetworkConfig type aliases (too many)"
    ((FAIL++))
fi
echo ""

# Check 6: Test if canonical NetworkConfig exists
echo "🔍 Check 6: Canonical NetworkConfig exists"
if [ -f "code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs" ]; then
    echo -e "  ${GREEN}✅ PASS${NC}: Canonical NetworkConfig file exists"
    ((PASS++))
else
    echo -e "  ${RED}❌ FAIL${NC}: Canonical NetworkConfig file NOT found"
    ((FAIL++))
fi
echo ""

# Check 7: Build status (optional, can be slow)
if [ "${CHECK_BUILD:-false}" = "true" ]; then
    echo "🔍 Check 7: Workspace build status"
    if cargo check --workspace --quiet 2>/dev/null; then
        echo -e "  ${GREEN}✅ PASS${NC}: Workspace builds successfully"
        ((PASS++))
    else
        echo -e "  ${RED}❌ FAIL${NC}: Workspace has build errors"
        ((FAIL++))
    fi
    echo ""
fi

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 VALIDATION SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "  ${GREEN}✅ Passed${NC}: $PASS checks"
echo -e "  ${YELLOW}⚠️  Warnings${NC}: $WARN checks"
echo -e "  ${RED}❌ Failed${NC}: $FAIL checks"
echo ""

# Overall status
if [ "$FAIL" -eq 0 ] && [ "$WARN" -eq 0 ]; then
    echo -e "${GREEN}╔══════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✅ MIGRATION COMPLETE - ALL CHECKS PASSED  ✅      ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════╝${NC}"
    exit 0
elif [ "$FAIL" -eq 0 ]; then
    echo -e "${YELLOW}╔══════════════════════════════════════════════════════╗${NC}"
    echo -e "${YELLOW}║  ⚠️  MIGRATION IN PROGRESS - WARNINGS PRESENT  ⚠️   ║${NC}"
    echo -e "${YELLOW}╚══════════════════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${RED}╔══════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║  ❌ MIGRATION INCOMPLETE - ISSUES FOUND  ❌         ║${NC}"
    echo -e "${RED}╚══════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "💡 Next steps:"
    echo "  1. Review failed checks above"
    echo "  2. Follow NETWORKCONFIG_MIGRATION_MAP.md"
    echo "  3. Run migration scripts or manual updates"
    echo "  4. Re-run this validation"
    exit 1
fi 