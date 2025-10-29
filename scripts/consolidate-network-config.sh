#!/bin/bash
# NetworkConfig Consolidation Script
# Date: September 30, 2025
# Purpose: Consolidate all NetworkConfig definitions to use CanonicalNetworkConfig

set -e

echo "🔍 NetworkConfig Consolidation Analysis"
echo "========================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Find the canonical definition
echo -e "${GREEN}✓ Canonical Definition:${NC}"
echo "  code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs"
echo "  Type: CanonicalNetworkConfig"
echo ""

# Find all NetworkConfig struct definitions
echo -e "${YELLOW}📋 NetworkConfig Definitions Found:${NC}"
grep -rn "pub struct NetworkConfig" code/crates --include="*.rs" | grep -v "// " | while read -r line; do
    echo "  $line"
done
echo ""

# Find all type aliases
echo -e "${YELLOW}📋 NetworkConfig Type Aliases:${NC}"
grep -rn "pub type NetworkConfig" code/crates --include="*.rs" | while read -r line; do
    file=$(echo "$line" | cut -d: -f1)
    if grep -q "CanonicalNetworkConfig" "$file"; then
        echo -e "  ${GREEN}✓${NC} $line (already using canonical)"
    else
        echo -e "  ${RED}✗${NC} $line (needs migration)"
    fi
done
echo ""

# Check imports
echo -e "${YELLOW}📋 Checking imports in key files:${NC}"
echo ""

# nestgate-network/src/lib.rs
echo "  nestgate-network/src/lib.rs:"
if grep -q "canonical_master::domains::network::CanonicalNetworkConfig" code/crates/nestgate-network/src/lib.rs 2>/dev/null; then
    echo -e "    ${GREEN}✓${NC} Already imports CanonicalNetworkConfig"
elif grep -q "CanonicalNetworkConfig" code/crates/nestgate-network/src/types.rs 2>/dev/null; then
    echo -e "    ${GREEN}✓${NC} Uses CanonicalNetworkConfig via types.rs"
else
    echo -e "    ${RED}✗${NC} Needs migration"
fi
echo ""

# nestgate-api/src/ecoprimal_sdk/config.rs
echo "  nestgate-api/src/ecoprimal_sdk/config.rs:"
if [ -f code/crates/nestgate-api/src/ecoprimal_sdk/config.rs ]; then
    if grep -q "canonical_master::domains::network" code/crates/nestgate-api/src/ecoprimal_sdk/config.rs; then
        echo -e "    ${GREEN}✓${NC} Already uses canonical"
    else
        echo -e "    ${RED}✗${NC} Has local NetworkConfig definition - needs migration"
    fi
else
    echo "    File not found"
fi
echo ""

# Check for usage issues
echo -e "${YELLOW}📋 Checking for incompatible field access:${NC}"
echo ""

# Common problematic patterns
echo "  Looking for .network. field access (doesn't exist on CanonicalNetworkConfig):"
grep -rn "\.network\." code/crates/nestgate-network/src --include="*.rs" | head -5 | while read -r line; do
    echo -e "    ${RED}✗${NC} $line"
done
echo ""

# Summary
echo -e "${GREEN}========================================${NC}"
echo "Summary:"
echo "  ✓ Canonical: domains/network/CanonicalNetworkConfig"
echo "  ✓ nestgate-network: Already using canonical (via type alias)"
echo "  ✗ nestgate-network/lib.rs: Has field access issues"
echo "  ✗ Other crates: Need to migrate remaining definitions"
echo ""
echo "Next Steps:"
echo "  1. Fix field access in nestgate-network/lib.rs"
echo "  2. Update nestgate-api/ecoprimal_sdk/config.rs"
echo "  3. Remove duplicate definitions in nestgate-core"
echo "  4. Update all imports to use canonical"
echo "" 