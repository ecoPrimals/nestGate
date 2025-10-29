#!/bin/bash
# StorageConfig Consolidation Script
# Date: September 30, 2025
# Purpose: Consolidate all StorageConfig definitions to use CanonicalStorageConfig

set -e

echo "🔍 StorageConfig Consolidation Analysis"
echo "========================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Find the canonical definition
echo -e "${GREEN}✓ Canonical Definition:${NC}"
echo "  code/crates/nestgate-core/src/config/canonical_master/domains/storage_canonical/"
echo "  Type: CanonicalStorageConfig"
echo ""

# Find all StorageConfig struct definitions
echo -e "${YELLOW}📋 StorageConfig Definitions Found:${NC}"
grep -rn "pub struct StorageConfig" code/crates --include="*.rs" | grep -v "// " | grep -v "StorageConfiguration" | while read -r line; do
    echo "  $line"
done
echo ""

# Find StorageConfiguration (variant spelling)
echo -e "${YELLOW}📋 StorageConfiguration Definitions Found:${NC}"
grep -rn "pub struct StorageConfiguration" code/crates --include="*.rs" | grep -v "// " | while read -r line; do
    echo "  $line"
done
echo ""

# Find all type aliases
echo -e "${YELLOW}📋 StorageConfig Type Aliases:${NC}"
grep -rn "pub type StorageConfig" code/crates --include="*.rs" | while read -r line; do
    file=$(echo "$line" | cut -d: -f1)
    if grep -q "CanonicalStorageConfig" "$file"; then
        echo -e "  ${GREEN}✓${NC} $line (already using canonical)"
    else
        echo -e "  ${RED}✗${NC} $line (needs migration)"
    fi
done
echo ""

# Check for common problematic patterns
echo -e "${YELLOW}📋 Checking for field access patterns:${NC}"
echo ""

# Look for .storage. field access
echo "  Looking for .storage. field access:"
if grep -rn "\.storage\." code/crates/nestgate-*/src --include="*.rs" 2>/dev/null | head -3 | grep -v test | grep -v comment; then
    echo -e "    ${YELLOW}⚠${NC} Found some .storage. field access patterns"
else
    echo -e "    ${GREEN}✓${NC} No problematic .storage. patterns found"
fi
echo ""

# Summary
echo -e "${GREEN}========================================${NC}"
echo "Summary:"
echo "  ✓ Canonical: domains/storage_canonical/CanonicalStorageConfig"
echo "  ? Duplicates found (see above)"
echo ""
echo "Next Steps:"
echo "  1. Review duplicate definitions"
echo "  2. Identify field access issues"
echo "  3. Create migration plan"
echo "  4. Follow NetworkConfig pattern"
echo "" 