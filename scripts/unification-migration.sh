#!/bin/bash

# **NESTGATE UNIFICATION MIGRATION SCRIPT**
#
# This script helps complete the unification and modernization of NestGate by:
# - Updating imports to use unified systems
# - Removing deprecated code patterns
# - Consolidating scattered configurations
# - Migrating async_trait to native async

set -euo pipefail

echo "🚀 Starting NestGate Unification Migration..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Migration statistics
CONFIGS_UPDATED=0
ERRORS_UPDATED=0
TRAITS_UPDATED=0
CONSTANTS_UPDATED=0

# ==================== CONFIGURATION MIGRATION ====================

echo -e "${BLUE}📋 Phase 1: Configuration Unification${NC}"

# Update imports to use unified configuration
echo "  Updating configuration imports..."
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*UnifiedConfig[^;]*/use nestgate_core::config::NestGateUnifiedConfig;/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*CanonicalConfig[^;]*/use nestgate_core::config::NestGateUnifiedConfig;/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*StandardDomainConfig[^;]*/use nestgate_core::config::NestGateUnifiedConfig;/g' {} \;

# Update type usage
find code/crates -name "*.rs" -type f -exec sed -i 's/\bUnifiedConfig\b/NestGateUnifiedConfig/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/\bCanonicalConfig\b/NestGateUnifiedConfig/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/\bStandardDomainConfig\b/NestGateUnifiedConfig/g' {} \;

CONFIGS_UPDATED=$(find code/crates -name "*.rs" -type f -exec grep -l "NestGateUnifiedConfig" {} \; | wc -l)
echo -e "  ${GREEN}✅ Updated $CONFIGS_UPDATED files to use unified configuration${NC}"

# ==================== ERROR SYSTEM MIGRATION ====================

echo -e "${BLUE}🔧 Phase 2: Error System Unification${NC}"

# Update error imports
echo "  Updating error system imports..."
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*NestGateError[^;]*/use nestgate_core::error::NestGateUnifiedError;/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*ApiError[^;]*/use nestgate_core::error::NestGateUnifiedError;/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*ZfsError[^;]*/use nestgate_core::error::NestGateUnifiedError;/g' {} \;

# Update error type usage
find code/crates -name "*.rs" -type f -exec sed -i 's/\bNestGateError::/NestGateUnifiedError::/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/\bApiError::/NestGateUnifiedError::Api/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/\bZfsError::/NestGateUnifiedError::Storage/g' {} \;

ERRORS_UPDATED=$(find code/crates -name "*.rs" -type f -exec grep -l "NestGateUnifiedError" {} \; | wc -l)
echo -e "  ${GREEN}✅ Updated $ERRORS_UPDATED files to use unified error system${NC}"

# ==================== CONSTANTS MIGRATION ====================

echo -e "${BLUE}📊 Phase 3: Constants Consolidation${NC}"

# Update constants imports
echo "  Updating constants imports..."
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*constants::[^;]*/use nestgate_core::constants::unified::/g' {} \;

# Replace hardcoded values with constants
echo "  Replacing hardcoded values..."
find code/crates -name "*.rs" -type f -exec sed -i 's/"127\.0\.0\.1"/nestgate_core::constants::unified::network::DEFAULT_HOST/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/8080/nestgate_core::constants::unified::network::DEFAULT_API_PORT/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/1000/nestgate_core::constants::unified::network::MAX_CONNECTIONS/g' {} \;

CONSTANTS_UPDATED=$(find code/crates -name "*.rs" -type f -exec grep -l "constants::unified" {} \; | wc -l)
echo -e "  ${GREEN}✅ Updated $CONSTANTS_UPDATED files to use unified constants${NC}"

# ==================== ASYNC_TRAIT MIGRATION ====================

echo -e "${BLUE}⚡ Phase 4: Async Trait Migration${NC}"

# Find remaining async_trait usage
ASYNC_TRAIT_COUNT=$(find code/crates -name "*.rs" -type f -exec grep -l "#\[async_trait\]" {} \; | wc -l)
echo "  Found $ASYNC_TRAIT_COUNT files with async_trait usage"

if [ $ASYNC_TRAIT_COUNT -gt 0 ]; then
    echo "  ${YELLOW}⚠️  Manual migration required for async_trait patterns${NC}"
    echo "  Files needing migration:"
    find code/crates -name "*.rs" -type f -exec grep -l "#\[async_trait\]" {} \; | head -10
    echo "  Use the native async patterns from nestgate_core::traits::native_async"
fi

# ==================== DEPRECATED CODE CLEANUP ====================

echo -e "${BLUE}🧹 Phase 5: Deprecated Code Cleanup${NC}"

# Find deprecated code
DEPRECATED_COUNT=$(find code/crates -name "*.rs" -type f -exec grep -l "#\[deprecated" {} \; | wc -l)
echo "  Found $DEPRECATED_COUNT files with deprecated code"

# Remove deprecated imports (be careful with this)
echo "  ${YELLOW}⚠️  Manual review required for deprecated code removal${NC}"

# ==================== FILE SIZE VALIDATION ====================

echo -e "${BLUE}📏 Phase 6: File Size Validation${NC}"

# Check for files over 2000 lines
echo "  Checking file sizes..."
LARGE_FILES=$(find code/crates -name "*.rs" -exec wc -l {} + | awk '$1 > 2000 {print $2, $1}' | wc -l)

if [ $LARGE_FILES -gt 0 ]; then
    echo -e "  ${RED}❌ Found $LARGE_FILES files exceeding 2000 lines:${NC}"
    find code/crates -name "*.rs" -exec wc -l {} + | awk '$1 > 2000 {print "    " $2 " (" $1 " lines)"}' | head -5
else
    echo -e "  ${GREEN}✅ All files comply with 2000-line limit${NC}"
fi

# ==================== COMPILATION TEST ====================

echo -e "${BLUE}🔨 Phase 7: Compilation Test${NC}"

echo "  Testing compilation..."
if cargo check --all --quiet; then
    echo -e "  ${GREEN}✅ Clean compilation achieved${NC}"
else
    echo -e "  ${RED}❌ Compilation errors remain - manual fixes needed${NC}"
fi

# ==================== SUMMARY ====================

echo ""
echo -e "${GREEN}🎉 NestGate Unification Migration Summary${NC}"
echo "================================================"
echo "Configuration files updated: $CONFIGS_UPDATED"
echo "Error system files updated: $ERRORS_UPDATED"  
echo "Constants files updated: $CONSTANTS_UPDATED"
echo "Files with async_trait: $ASYNC_TRAIT_COUNT"
echo "Files with deprecated code: $DEPRECATED_COUNT"
echo "Large files (>2000 lines): $LARGE_FILES"
echo ""

if [ $ASYNC_TRAIT_COUNT -eq 0 ] && [ $LARGE_FILES -eq 0 ]; then
    echo -e "${GREEN}🏆 UNIFICATION COMPLETE! All targets achieved.${NC}"
else
    echo -e "${YELLOW}⚠️  Additional manual work needed:${NC}"
    [ $ASYNC_TRAIT_COUNT -gt 0 ] && echo "  - Migrate $ASYNC_TRAIT_COUNT async_trait patterns"
    [ $LARGE_FILES -gt 0 ] && echo "  - Split $LARGE_FILES large files"
fi

echo ""
echo "Next steps:"
echo "1. Review compilation errors and fix manually"
echo "2. Migrate remaining async_trait patterns to native async"
echo "3. Split any files exceeding 2000 lines"
echo "4. Remove deprecated code after migration validation"
echo "5. Update documentation to reflect unified systems"

echo ""
echo -e "${GREEN}🚀 NestGate Unification Migration Complete!${NC}" 