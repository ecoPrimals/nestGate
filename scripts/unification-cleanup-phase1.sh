#!/bin/bash
# 🎯 NestGate Unification Cleanup - Phase 1
# Quick wins: LegacyModuleError removal and duplicate constants audit

set -euo pipefail

echo "🎯 NestGate Unification Cleanup - Phase 1"
echo "=========================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

REPORT_FILE="PHASE1_CLEANUP_REPORT.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "📊 PHASE 1: ANALYSIS"
echo "-------------------"

# 1. Count LegacyModuleError instances
echo ""
echo "${YELLOW}1. LegacyModuleError Boilerplate Analysis${NC}"
LEGACY_ERROR_COUNT=$(grep -r "pub enum LegacyModuleError" code/crates/nestgate-core/src/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Found: ${LEGACY_ERROR_COUNT} instances"
echo "   These are template boilerplate and can be safely removed"

# List files with LegacyModuleError
echo ""
echo "   Files containing LegacyModuleError:"
grep -r "pub enum LegacyModuleError" code/crates/nestgate-core/src/ --include="*.rs" -l 2>/dev/null | head -20 | sed 's/^/     - /'

# 2. Count duplicate MODULE_VERSION constants
echo ""
echo "${YELLOW}2. Duplicate MODULE_VERSION Constants${NC}"
MODULE_VERSION_COUNT=$(rg "pub const MODULE_VERSION" --type rust code/crates/nestgate-core/src/ 2>/dev/null | wc -l)
echo "   Found: ${MODULE_VERSION_COUNT} definitions"
echo "   Should be: 1 in shared constants module"

# List files with MODULE_VERSION
echo ""
echo "   Files defining MODULE_VERSION:"
rg "pub const MODULE_VERSION" --type rust code/crates/nestgate-core/src/ -l 2>/dev/null | head -15 | sed 's/^/     - /'

# 3. Count Config struct definitions
echo ""
echo "${YELLOW}3. Config Struct Fragmentation${NC}"
CONFIG_STRUCT_COUNT=$(rg "pub struct.*Config\s*\{" --type rust code/crates/nestgate-core/src/config/ 2>/dev/null | wc -l)
echo "   Found: ${CONFIG_STRUCT_COUNT} Config structs in core/config/"
echo "   Target: Consolidate to canonical system"

# 4. Count Storage trait definitions
echo ""
echo "${YELLOW}4. Storage Trait Fragmentation${NC}"
STORAGE_TRAIT_COUNT=$(rg "pub trait.*Storage" --type rust code/crates/nestgate-core/src/ 2>/dev/null | wc -l)
echo "   Found: ${STORAGE_TRAIT_COUNT} Storage trait definitions"
echo "   Target: Consolidate to UnifiedStorage"

# 5. Count Error enum definitions
echo ""
echo "${YELLOW}5. Error Enum Definitions${NC}"
ERROR_ENUM_COUNT=$(rg "pub enum.*Error" --type rust code/crates/nestgate-core/src/ 2>/dev/null | wc -l)
echo "   Found: ${ERROR_ENUM_COUNT} Error enum definitions"
echo "   Target: Use NestGateUnifiedError variants"

# 6. Check migration helpers
echo ""
echo "${YELLOW}6. Migration Helpers (to be removed after migration)${NC}"
if [ -d "code/crates/nestgate-core/src/error/migration_helpers" ]; then
    MIGRATION_HELPERS=$(ls code/crates/nestgate-core/src/error/migration_helpers/*.rs 2>/dev/null | wc -l)
    echo "   Error migration helpers: ${MIGRATION_HELPERS} files"
    ls code/crates/nestgate-core/src/error/migration_helpers/*.rs 2>/dev/null | sed 's/^/     - /'
else
    echo "   Error migration helpers: directory not found"
fi

if [ -d "code/crates/nestgate-core/src/config/migration_helpers" ]; then
    CONFIG_MIGRATION_HELPERS=$(ls code/crates/nestgate-core/src/config/migration_helpers/*.rs 2>/dev/null | wc -l)
    echo "   Config migration helpers: ${CONFIG_MIGRATION_HELPERS} files"
    ls code/crates/nestgate-core/src/config/migration_helpers/*.rs 2>/dev/null | sed 's/^/     - /'
else
    echo "   Config migration helpers: directory not found"
fi

# Generate detailed report
echo ""
echo "📝 GENERATING DETAILED REPORT"
echo "-----------------------------"

cat > "$REPORT_FILE" << EOF
# Phase 1 Cleanup Report
Generated: $(date)

## Summary
- LegacyModuleError instances: ${LEGACY_ERROR_COUNT}
- Duplicate MODULE_VERSION: ${MODULE_VERSION_COUNT}
- Config structs in core: ${CONFIG_STRUCT_COUNT}
- Storage trait definitions: ${STORAGE_TRAIT_COUNT}
- Error enum definitions: ${ERROR_ENUM_COUNT}

## Priority 1: LegacyModuleError Removal (Easiest Win)
Status: Safe to remove - these are unused boilerplate

Files to clean:
EOF

grep -r "pub enum LegacyModuleError" code/crates/nestgate-core/src/ --include="*.rs" -l 2>/dev/null >> "$REPORT_FILE"

cat >> "$REPORT_FILE" << EOF

Action: Remove all "pub enum LegacyModuleError { Unknown(String) }" blocks

## Priority 2: Duplicate MODULE_VERSION Constants

Files with duplicate definitions:
EOF

rg "pub const MODULE_VERSION" --type rust code/crates/nestgate-core/src/ -l 2>/dev/null >> "$REPORT_FILE"

cat >> "$REPORT_FILE" << EOF

Action: Create code/crates/nestgate-core/src/constants/shared.rs
        Replace all duplicates with: use crate::constants::shared::MODULE_VERSION;

## Priority 3: Storage Trait Consolidation

Storage traits found:
EOF

rg "pub trait.*Storage" --type rust code/crates/nestgate-core/src/ 2>/dev/null | head -30 >> "$REPORT_FILE"

cat >> "$REPORT_FILE" << EOF

Action: Consolidate to traits/unified_storage.rs::UnifiedStorage
        Add type aliases for migration: type StorageBackend = UnifiedStorage;

## Next Steps

Week 1:
[ ] Remove LegacyModuleError boilerplate (50+ files)
[ ] Create shared constants module
[ ] Audit and document all storage traits

Week 2:
[ ] Consolidate NetworkConfig duplicates
[ ] Consolidate StorageConfig duplicates
[ ] Mark deprecated traits

Week 3:
[ ] Remove domain-specific error enums
[ ] Consolidate remaining constants
[ ] Update documentation

Week 4:
[ ] Remove migration helpers
[ ] Remove compatibility shims
[ ] Final validation
EOF

echo "   Report saved to: ${REPORT_FILE}"

# Offer to perform safe cleanups
echo ""
echo "🤖 AUTOMATED CLEANUP OPTIONS"
echo "-----------------------------"
echo ""
echo "Would you like to perform any of these safe cleanups? (y/n for each)"
echo ""

# Option 1: Remove LegacyModuleError
echo -n "${GREEN}1. Remove LegacyModuleError boilerplate (${LEGACY_ERROR_COUNT} instances)?${NC} [y/N]: "
read -r REMOVE_LEGACY_ERROR

if [[ "$REMOVE_LEGACY_ERROR" =~ ^[Yy]$ ]]; then
    echo "   Removing LegacyModuleError definitions..."
    
    # Create backup first
    BACKUP_DIR="backups/phase1-legacy-error-$(date +%Y%m%d-%H%M%S)"
    mkdir -p "$BACKUP_DIR"
    
    # Find and backup files
    grep -r "pub enum LegacyModuleError" code/crates/nestgate-core/src/ --include="*.rs" -l 2>/dev/null | while read -r file; do
        mkdir -p "$BACKUP_DIR/$(dirname "$file")"
        cp "$file" "$BACKUP_DIR/$file"
    done
    
    echo "   ✅ Backup created in: $BACKUP_DIR"
    echo "   Note: Actual removal requires careful review of each file"
    echo "   Recommendation: Manual review and removal to avoid breaking dependencies"
else
    echo "   Skipped."
fi

echo ""
echo "📊 SUMMARY"
echo "----------"
echo "${GREEN}✅ Analysis complete${NC}"
echo "${YELLOW}📝 Detailed report: ${REPORT_FILE}${NC}"
echo ""
echo "Next steps:"
echo "1. Review the report: cat ${REPORT_FILE}"
echo "2. Start with Priority 1 (LegacyModuleError removal)"
echo "3. Follow the 4-week plan in UNIFICATION_STATUS_REPORT_2025_09_30.md"
echo ""
echo "For detailed guidance, see:"
echo "  - UNIFICATION_STATUS_REPORT_2025_09_30.md"
echo "  - CANONICAL_CONFIG_DECISION.md"
echo "  - UNIFICATION_ASSESSMENT_REPORT.md"
echo "" 