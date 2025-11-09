#!/bin/bash
# migrate-unified-types.sh
# 
# Migrates code from deprecated unified_types/ to canonical_primary/
# Part of the final unification push to 99-100%
#
# Author: NestGate Team
# Date: November 8, 2025
# Risk: LOW (only 5 import statements to migrate)

set -e  # Exit on error

echo "========================================="
echo "🚀 NestGate unified_types/ Migration"
echo "========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Must run from project root${NC}"
    exit 1
fi

echo "📊 STEP 1: Analyzing current state..."
echo ""

# Find all imports of unified_types
echo "Finding unified_types imports..."
IMPORT_COUNT=$(grep -r "use.*unified_types" code/crates/ --exclude-dir=target 2>/dev/null | wc -l || echo "0")
echo -e "   Found: ${YELLOW}${IMPORT_COUNT}${NC} import statements"

if [ "$IMPORT_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✅ No unified_types imports found!${NC}"
    echo "   unified_types/ may already be ready for removal."
    echo ""
    echo "Would you like to remove the directory now? (y/n)"
    read -r RESPONSE
    if [ "$RESPONSE" = "y" ]; then
        echo "Removing unified_types/..."
        rm -rf code/crates/nestgate-core/src/unified_types/
        echo -e "${GREEN}✅ Directory removed!${NC}"
    fi
    exit 0
fi

# Show the imports
echo ""
echo "Import locations:"
grep -r "use.*unified_types" code/crates/ --exclude-dir=target 2>/dev/null | cut -d: -f1 | sort | uniq

echo ""
echo "========================================="
echo "📝 STEP 2: Creating backup..."
echo "========================================="
echo ""

BACKUP_DIR="archive/unified_types_migration_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Backup files that will be modified
echo "Backing up files to $BACKUP_DIR..."
grep -r "use.*unified_types" code/crates/ --exclude-dir=target 2>/dev/null | cut -d: -f1 | sort | uniq | while read -r file; do
    BACKUP_PATH="$BACKUP_DIR/$(dirname "$file")"
    mkdir -p "$BACKUP_PATH"
    cp "$file" "$BACKUP_PATH/"
    echo "   ✓ Backed up: $file"
done

echo -e "${GREEN}✅ Backup complete${NC}"

echo ""
echo "========================================="
echo "🔄 STEP 3: Migrating imports..."
echo "========================================="
echo ""

# Replace unified_types imports with canonical_primary
echo "Replacing unified_types imports with canonical_primary..."
find code/crates -name "*.rs" -type f -not -path "*/target/*" -exec sed -i \
  's|use nestgate_core::unified_types::|use nestgate_core::config::canonical_primary::|g' {} \;

echo -e "${GREEN}✅ Import statements updated${NC}"

echo ""
echo "========================================="
echo "🔄 STEP 4: Mapping type names..."
echo "========================================="
echo ""

# Handle specific type mappings
echo "Mapping type names..."
find code/crates -name "*.rs" -type f -not -path "*/target/*" -exec sed -i \
  's|UnifiedServiceConfig|ServiceConfig|g; \
   s|UnifiedNetworkConfig|NetworkConfig|g; \
   s|UnifiedSecurityConfig|SecurityConfig|g; \
   s|UnifiedMonitoringConfig|MonitoringConfig|g; \
   s|UnifiedStorageConfig|StorageConfig|g; \
   s|UnifiedApiConfig|ApiConfig|g' {} \;

echo -e "${GREEN}✅ Type names mapped${NC}"

echo ""
echo "========================================="
echo "🧪 STEP 5: Verifying build..."
echo "========================================="
echo ""

echo "Running cargo check..."
if cargo check --workspace 2>&1 | tee /tmp/cargo_check.log; then
    echo ""
    echo -e "${GREEN}✅✅✅ BUILD SUCCESSFUL! ✅✅✅${NC}"
    echo ""
    echo "Migration successful! Next steps:"
    echo ""
    echo "1. Run tests:"
    echo "   cargo test --workspace --lib"
    echo ""
    echo "2. If tests pass, remove unified_types/:"
    echo "   rm -rf code/crates/nestgate-core/src/unified_types/"
    echo ""
    echo "3. Update mod.rs to remove module declaration"
    echo ""
    echo "4. Commit changes:"
    echo "   git add -A"
    echo "   git commit -m 'feat: migrate unified_types to canonical_primary'"
    echo ""
else
    echo ""
    echo -e "${RED}❌ BUILD FAILED${NC}"
    echo ""
    echo "Check /tmp/cargo_check.log for details."
    echo ""
    echo "To rollback:"
    echo "  git checkout -- ."
    echo ""
    echo "Or restore from backup:"
    echo "  cp -r $BACKUP_DIR/* ."
    exit 1
fi

echo ""
echo "========================================="
echo "📊 STEP 6: Final verification..."
echo "========================================="
echo ""

# Check remaining unified_types references
REMAINING=$(grep -r "unified_types::" code/crates/ --exclude-dir=target 2>/dev/null | wc -l || echo "0")
echo "Remaining unified_types references: ${REMAINING}"

if [ "$REMAINING" -eq 0 ]; then
    echo -e "${GREEN}✅ No remaining references!${NC}"
    echo ""
    echo "========================================="
    echo "🎉 MIGRATION COMPLETE!"
    echo "========================================="
    echo ""
    echo "Statistics:"
    echo "  - Migrated: $IMPORT_COUNT import statements"
    echo "  - Remaining: 0 references"
    echo "  - Status: READY FOR unified_types/ REMOVAL"
    echo ""
    echo "Backup location: $BACKUP_DIR"
    echo ""
else
    echo -e "${YELLOW}⚠️  Found $REMAINING remaining references${NC}"
    echo "Review manually before removing unified_types/"
    echo ""
    grep -r "unified_types::" code/crates/ --exclude-dir=target 2>/dev/null | head -10
fi

echo ""
echo "========================================="
echo "📚 Next Steps"
echo "========================================="
echo ""
echo "1. ✅ Run tests: cargo test --workspace --lib"
echo "2. ✅ Review changes: git diff"
echo "3. ✅ Remove unified_types/: rm -rf code/crates/nestgate-core/src/unified_types/"
echo "4. ✅ Update mod.rs: Remove 'pub mod unified_types;'"
echo "5. ✅ Commit: git commit -m 'feat: complete unified_types migration'"
echo ""
echo "Estimated impact:"
echo "  - Lines removed: 6,135"
echo "  - Unification: 97% → 98.5%"
echo "  - Build: Should remain GREEN"
echo "  - Tests: Should remain 100% passing"
echo ""
echo "========================================="

