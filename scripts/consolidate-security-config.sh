#!/bin/bash
# SecurityConfig Consolidation Script
# Date: September 30, 2025
# Purpose: Consolidate all SecurityConfig definitions to use CanonicalSecurityConfig

set -e

echo "🔍 SecurityConfig Consolidation Analysis"
echo "========================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Find the canonical definition
echo -e "${GREEN}✓ Canonical Definition:${NC}"
echo "  code/crates/nestgate-core/src/config/canonical_master/domains/security_canonical/"
echo "  Type: CanonicalSecurityConfig"
echo ""

# Find all SecurityConfig struct definitions
echo -e "${YELLOW}📋 SecurityConfig Definitions Found:${NC}"
grep -rn "pub struct SecurityConfig" code/crates --include="*.rs" | grep -v "// " | head -15 | while read -r line; do
    echo "  $line"
done
echo ""

# Find SecuritySettings/SecurityConfiguration variants
echo -e "${YELLOW}📋 Security* Variant Definitions Found:${NC}"
grep -rn "pub struct Security\(Settings\|Configuration\)" code/crates --include="*.rs" | head -10 | while read -r line; do
    echo "  $line"
done
echo ""

# Find all type aliases
echo -e "${YELLOW}📋 SecurityConfig Type Aliases:${NC}"
grep -rn "pub type SecurityConfig" code/crates --include="*.rs" | while read -r line; do
    file=$(echo "$line" | cut -d: -f1)
    if grep -q "CanonicalSecurityConfig" "$file"; then
        echo -e "  ${GREEN}✓${NC} $line (already using canonical)"
    else
        echo -e "  ${RED}✗${NC} $line (needs migration)"
    fi
done
echo ""

# Count total definitions
TOTAL_DEFS=$(grep -rn "pub struct Security" code/crates --include="*.rs" | grep -i config | wc -l)

echo -e "${GREEN}========================================${NC}"
echo "Summary:"
echo "  ✓ Canonical: domains/security_canonical/CanonicalSecurityConfig"
echo "  📊 Total definitions found: $TOTAL_DEFS"
echo ""
echo "Next Steps:"
echo "  1. Add deprecation markers"
echo "  2. Follow proven pattern"
echo "  3. Validate builds"
echo "" 