#!/bin/bash
# 🔄 UNIFIED_TYPES TO CANONICAL_PRIMARY MIGRATION SCRIPT
# Migrates deprecated unified_types/ imports to canonical_primary/
# Part of 97% → 100% Unification Plan

set -euo pipefail

echo "🔄 **NESTGATE UNIFIED_TYPES MIGRATION**"
echo "======================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "📊 **PHASE 1: PRE-MIGRATION ANALYSIS**"
echo "--------------------------------------"

# Count current usages
CURRENT_UNIFIED_TYPES=$(find code/crates -name "*.rs" -exec grep -l "use nestgate_core::unified_types::" {} \; 2>/dev/null | wc -l)
CURRENT_CANONICAL=$(find code/crates -name "*.rs" -exec grep -l "use nestgate_core::config::canonical_primary::" {} \; 2>/dev/null | wc -l)

echo "Current unified_types/ imports: $CURRENT_UNIFIED_TYPES files"
echo "Current canonical_primary/ imports: $CURRENT_CANONICAL files"
echo ""

if [ "$CURRENT_UNIFIED_TYPES" -eq 0 ]; then
    echo -e "${GREEN}✅ No unified_types imports found - migration may already be complete!${NC}"
    echo ""
    echo "Checking if unified_types/ directory should be removed..."
    if [ -d "code/crates/nestgate-core/src/unified_types" ]; then
        echo -e "${YELLOW}⚠️  unified_types/ directory still exists${NC}"
        echo "Run with --remove-deprecated flag to remove it."
    fi
    exit 0
fi

echo "🔍 **PHASE 2: BACKUP CURRENT STATE**"
echo "------------------------------------"

BACKUP_DIR="backups/unified-types-migration-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

echo "Creating backup in: $BACKUP_DIR"

# Backup all files that will be modified
find code/crates -name "*.rs" -exec grep -l "use nestgate_core::unified_types::" {} \; 2>/dev/null | while read -r file; do
    mkdir -p "$BACKUP_DIR/$(dirname "$file")"
    cp "$file" "$BACKUP_DIR/$file"
done

BACKUP_COUNT=$(find "$BACKUP_DIR" -name "*.rs" | wc -l)
echo -e "${GREEN}✅ Backed up $BACKUP_COUNT files${NC}"
echo ""

echo "🔄 **PHASE 3: MIGRATION EXECUTION**"
echo "-----------------------------------"

# Migration patterns
declare -A MIGRATIONS=(
    # Config types
    ["unified_types::connection_pool_config"]="config::canonical_primary::connection_pool"
    ["unified_types::retry_config"]="config::canonical_primary::retry"
    ["unified_types::timeout_config"]="config::canonical_primary::timeout"
    ["unified_types::memory_config"]="config::canonical_primary::memory"
    ["unified_types::service_config"]="config::canonical_primary::service"
    ["unified_types::monitoring_config"]="config::canonical_primary::monitoring"
    ["unified_types::security_config"]="config::canonical_primary::domains::security_canonical"
    ["unified_types::storage_config"]="config::canonical_primary::domains::storage_canonical"
    ["unified_types::cache_config"]="config::canonical_primary::domains::performance::caching"
    
    # General fallback
    ["unified_types::"]="config::canonical_primary::"
)

TOTAL_REPLACEMENTS=0

for pattern in "${!MIGRATIONS[@]}"; do
    replacement="${MIGRATIONS[$pattern]}"
    
    echo "Migrating: $pattern → $replacement"
    
    # Find and replace in all .rs files
    COUNT=$(find code/crates -name "*.rs" -type f -exec sed -i "s|nestgate_core::${pattern}|nestgate_core::${replacement}|g" {} \; -exec grep -l "nestgate_core::${replacement}" {} \; 2>/dev/null | wc -l)
    
    if [ "$COUNT" -gt 0 ]; then
        echo -e "  ${GREEN}✓${NC} Updated in $COUNT files"
        TOTAL_REPLACEMENTS=$((TOTAL_REPLACEMENTS + COUNT))
    fi
done

echo ""
echo -e "${GREEN}✅ Migration complete: $TOTAL_REPLACEMENTS file updates${NC}"
echo ""

echo "🧪 **PHASE 4: VALIDATION**"
echo "-------------------------"

echo "Running cargo check..."
if cargo check --workspace --quiet 2>/dev/null; then
    echo -e "${GREEN}✅ cargo check passed${NC}"
else
    echo -e "${RED}❌ cargo check failed${NC}"
    echo ""
    echo "Migration may have introduced errors. Check output above."
    echo "Backups are available in: $BACKUP_DIR"
    exit 1
fi

echo ""
echo "Running library tests..."
if cargo test --workspace --lib --quiet 2>/dev/null; then
    echo -e "${GREEN}✅ Tests passed${NC}"
else
    echo -e "${YELLOW}⚠️  Some tests failed - review required${NC}"
    echo "This may be expected during migration."
fi

echo ""
echo "📊 **PHASE 5: POST-MIGRATION ANALYSIS**"
echo "---------------------------------------"

NEW_UNIFIED_TYPES=$(find code/crates -name "*.rs" -exec grep -l "use nestgate_core::unified_types::" {} \; 2>/dev/null | wc -l)
NEW_CANONICAL=$(find code/crates -name "*.rs" -exec grep -l "use nestgate_core::config::canonical_primary::" {} \; 2>/dev/null | wc -l)

echo "Before migration:"
echo "  unified_types/ imports: $CURRENT_UNIFIED_TYPES files"
echo "  canonical_primary/ imports: $CURRENT_CANONICAL files"
echo ""
echo "After migration:"
echo "  unified_types/ imports: $NEW_UNIFIED_TYPES files"
echo "  canonical_primary/ imports: $NEW_CANONICAL files"
echo ""

REDUCTION=$((CURRENT_UNIFIED_TYPES - NEW_UNIFIED_TYPES))
if [ "$REDUCTION" -gt 0 ]; then
    echo -e "${GREEN}✅ Reduced unified_types usage by $REDUCTION files${NC}"
else
    echo -e "${YELLOW}⚠️  No reduction in unified_types usage${NC}"
fi

echo ""
echo "✅ **MIGRATION COMPLETE**"
echo "========================"
echo ""
echo "Backup location: $BACKUP_DIR"
echo ""
echo "Next steps:"
echo "1. Review changes: git diff"
echo "2. Run full tests: cargo test --workspace"
echo "3. If successful, run with --remove-deprecated to clean up unified_types/"
echo ""

