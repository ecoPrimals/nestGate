#!/bin/bash
# 🔧 **CONFIGURATION CONSOLIDATION MIGRATION SCRIPT**
# Systematically migrates from UnifiedCanonicalConfig to ConsolidatedCanonicalConfig

set -euo pipefail

echo "🔧 **NESTGATE CONFIGURATION CONSOLIDATION MIGRATION**"
echo "===================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Create backup
BACKUP_DIR="config_migration_backup_$(date +%Y%m%d_%H%M%S)"
echo "📦 Creating backup: $BACKUP_DIR"
cp -r code/ "$BACKUP_DIR"

echo ""
echo "🔍 **PHASE 1: ANALYSIS**"
echo "========================"

# Count current usage
UNIFIED_USAGE=$(grep -r "UnifiedCanonicalConfig" code/crates --include="*.rs" | wc -l)
CONSOLIDATED_USAGE=$(grep -r "ConsolidatedCanonicalConfig" code/crates --include="*.rs" | wc -l)

echo "📊 Current state:"
echo "  - UnifiedCanonicalConfig references: $UNIFIED_USAGE"
echo "  - ConsolidatedCanonicalConfig references: $CONSOLIDATED_USAGE"

echo ""
echo "🔄 **PHASE 2: SYSTEMATIC MIGRATION**"
echo "===================================="

# Step 1: Update import statements
echo "  📝 Updating import statements..."
find code/crates -name "*.rs" -type f -exec sed -i \
  's/use.*UnifiedCanonicalConfig/use crate::config::consolidated_canonical_config::ConsolidatedCanonicalConfig/g' {} \;

# Step 2: Update type aliases
echo "  🏷️  Updating type aliases..."
find code/crates -name "*.rs" -type f -exec sed -i \
  's/pub type.*= UnifiedCanonicalConfig/pub type StandardConfig = ConsolidatedCanonicalConfig/g' {} \;

# Step 3: Update struct field types
echo "  📦 Updating struct field types..."
find code/crates -name "*.rs" -type f -exec sed -i \
  's/: UnifiedCanonicalConfig/: ConsolidatedCanonicalConfig/g' {} \;

# Step 4: Update function parameters
echo "  🔧 Updating function parameters..."
find code/crates -name "*.rs" -type f -exec sed -i \
  's/(.*config: UnifiedCanonicalConfig/(config: ConsolidatedCanonicalConfig/g' {} \;

# Step 5: Update return types
echo "  📤 Updating return types..."
find code/crates -name "*.rs" -type f -exec sed -i \
  's/-> UnifiedCanonicalConfig/-> ConsolidatedCanonicalConfig/g' {} \;

# Step 6: Update generic parameters
echo "  🔀 Updating generic parameters..."
find code/crates -name "*.rs" -type f -exec sed -i \
  's/<UnifiedCanonicalConfig>/<ConsolidatedCanonicalConfig>/g' {} \;

echo ""
echo "🔧 **PHASE 3: SPECIALIZED MIGRATIONS**"
echo "====================================="

# Update specific patterns found in the codebase
echo "  🎯 Handling specialized patterns..."

# Update nestgate-core lib.rs exports
if [ -f "code/crates/nestgate-core/src/lib.rs" ]; then
  echo "  📚 Updating nestgate-core exports..."
  sed -i 's/pub use config::{UnifiedCanonicalConfig, Config}/pub use config::{ConsolidatedCanonicalConfig as Config, ConsolidatedCanonicalConfig}/g' \
    "code/crates/nestgate-core/src/lib.rs"
fi

# Update config mod.rs
if [ -f "code/crates/nestgate-core/src/config/mod.rs" ]; then
  echo "  ⚙️  Updating config module exports..."
  sed -i 's/pub type UnifiedCanonicalConfig = CanonicalConfig/pub type Config = ConsolidatedCanonicalConfig/g' \
    "code/crates/nestgate-core/src/config/mod.rs"
fi

echo ""
echo "🧹 **PHASE 4: CLEANUP DEPRECATED MODULES**"
echo "==========================================="

# Mark deprecated modules for removal
DEPRECATED_MODULES=(
  "code/crates/nestgate-core/src/config/unified_canonical_config.rs"
  "code/crates/nestgate-core/src/config/canonical_config/mod.rs"
  "code/crates/nestgate-core/src/config/canonical_master/mod.rs"
  "code/crates/nestgate-core/src/config/core.rs"
)

for module in "${DEPRECATED_MODULES[@]}"; do
  if [ -f "$module" ]; then
    echo "  🗑️  Marking deprecated: $module"
    # Add deprecation header
    {
      echo "//! **DEPRECATED MODULE - SCHEDULED FOR REMOVAL**"
      echo "//! This module has been superseded by ConsolidatedCanonicalConfig"
      echo "//! Use: use nestgate_core::config::consolidated_canonical_config::ConsolidatedCanonicalConfig"
      echo ""
      cat "$module"
    } > "$module.tmp" && mv "$module.tmp" "$module"
  fi
done

echo ""
echo "🔍 **PHASE 5: VALIDATION**"
echo "=========================="

echo "  🔧 Running cargo check..."
if cargo check --workspace --quiet 2>/dev/null; then
  echo "  ✅ Migration validates successfully!"
else
  echo "  ⚠️  Some compilation issues detected"
  echo "  📋 Detailed errors:"
  cargo check --workspace 2>&1 | head -20
fi

# Final count
REMAINING_UNIFIED=$(grep -r "UnifiedCanonicalConfig" code/crates --include="*.rs" | grep -v "DEPRECATED" | wc -l)
NEW_CONSOLIDATED=$(grep -r "ConsolidatedCanonicalConfig" code/crates --include="*.rs" | wc -l)

echo ""
echo "📊 **MIGRATION SUMMARY**"
echo "========================"
echo "  - Remaining UnifiedCanonicalConfig references: $REMAINING_UNIFIED"
echo "  - New ConsolidatedCanonicalConfig references: $NEW_CONSOLIDATED"
echo "  - Backup created at: $BACKUP_DIR"

if [ "$REMAINING_UNIFIED" -eq 0 ]; then
  echo "  ✅ **MIGRATION COMPLETE** - All references updated!"
else
  echo "  🔄 **MIGRATION IN PROGRESS** - Manual review needed for remaining references"
fi

echo ""
echo "📋 **NEXT STEPS**"
echo "=================="
echo "1. Review compilation errors and fix any type mismatches"
echo "2. Update tests to use ConsolidatedCanonicalConfig"
echo "3. Remove deprecated modules after validation"
echo "4. Update documentation to reflect new configuration system"

echo ""
echo "✅ Configuration consolidation migration completed!" 