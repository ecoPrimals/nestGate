#!/bin/bash
# remove-deprecated-modules.sh
# Removes deprecated config modules that are commented out and causing errors

set -e

echo "🗑️  Removing deprecated config modules..."
echo ""

cd "$(dirname "$0")/.."

# Create backup
BACKUP_DIR="backups/deprecated-removal-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "💾 Creating backup in $BACKUP_DIR/"
find code/crates/nestgate-core/src/config -type f -name "*.rs" -exec cp --parents {} "$BACKUP_DIR/" \; 2>/dev/null
echo "✅ Backup complete"
echo ""

echo "🔍 Deprecated modules to remove:"
echo "   - canonical (use canonical_master)"
echo "   - canonical_config (use canonical_master)"
echo "   - canonical_unified (use canonical_master)"
echo "   - unified_types (use canonical_master)"
echo "   - domains (merged into canonical_master)"
echo "   - monitoring (merged into canonical_master)"
echo "   - network (merged into canonical_master)"
echo "   - security (merged into canonical_master)"
echo "   - storage (merged into canonical_master)"
echo ""

# Remove the deprecated module directories
MODULES_TO_REMOVE=(
    "code/crates/nestgate-core/src/config/canonical"
    "code/crates/nestgate-core/src/config/canonical_config"
    "code/crates/nestgate-core/src/config/canonical_unified"
    "code/crates/nestgate-core/src/config/unified_types"
    "code/crates/nestgate-core/src/config/domains"
    "code/crates/nestgate-core/src/config/monitoring"
    "code/crates/nestgate-core/src/config/network"
    "code/crates/nestgate-core/src/config/security"
    "code/crates/nestgate-core/src/config/storage"
)

for module_dir in "${MODULES_TO_REMOVE[@]}"; do
    if [ -d "$module_dir" ]; then
        echo "🗑️  Removing $module_dir"
        rm -rf "$module_dir"
    else
        echo "⏭️  Skipping $module_dir (doesn't exist)"
    fi
done

echo ""
echo "📝 Cleaning up mod.rs..."

# Remove the commented-out module declarations from mod.rs
# We'll create a clean version
cat > code/crates/nestgate-core/src/config/mod.rs << 'EOF'
//! Configuration management module
//!
//! This module provides the unified configuration system using canonical_master as the single source of truth.

// ============================================================================
// CANONICAL CONFIGURATION - SINGLE SOURCE OF TRUTH
// ============================================================================

/// Canonical master configuration - the one and only config system
pub mod canonical_master;
pub use canonical_master::NestGateCanonicalConfig;

// ============================================================================
// MIGRATION HELPERS (Temporary - Week 2-3 cleanup)
// ============================================================================

/// Migration helpers for transitioning from legacy config systems
pub mod migration_helpers;

// Re-export commonly used types
pub use canonical_master::{
    SystemConfig,
    StoragePoolConfig,
};
EOF

echo "✅ mod.rs cleaned up"
echo ""

# Check build
echo "🔍 Checking build..."
if timeout 90 cargo check --package nestgate-core 2>&1 | grep "^error:" | tail -1 > /tmp/error-count.txt; then
    cat /tmp/error-count.txt
else
    echo "✅ Build check complete"
fi

echo ""
echo "📂 Backup: $BACKUP_DIR/"
echo ""
echo "💡 This should eliminate many E0425 (unresolved name) and E0433 (unresolved import) errors" 