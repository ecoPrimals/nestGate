#!/bin/bash
# migrate-networkconfig-to-canonical.sh
# Migrates NetworkConfig usage from old unified system to canonical_master

set -euo pipefail

echo "🔄 **NETWORKCONFIG CANONICAL MIGRATION**"
echo "========================================"
echo ""

cd "$(dirname "$0")/.."

TARGET_CRATE="${1:-code/crates/nestgate-network}"

if [ ! -d "$TARGET_CRATE" ]; then
    echo "❌ Error: Crate directory not found: $TARGET_CRATE"
    exit 1
fi

echo "📦 Target crate: $TARGET_CRATE"
echo ""

# Create backup
BACKUP_DIR="backups/networkconfig-migration-$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "📦 Creating backup in $BACKUP_DIR..."
cp -r "$TARGET_CRATE" "$BACKUP_DIR/"
echo "   ✅ Backup created"
echo ""

echo "🔍 **STEP 1**: Analyzing current NetworkConfig usage..."
NETWORKCONFIG_USAGE=$(grep -r "NetworkConfig" "$TARGET_CRATE/src" --include="*.rs" | wc -l)
echo "   Found $NETWORKCONFIG_USAGE occurrences of NetworkConfig"
echo ""

echo "📝 **STEP 2**: Updating type aliases and imports..."

# Update types.rs to use CanonicalNetworkConfig
TYPES_FILE="$TARGET_CRATE/src/types.rs"
if [ -f "$TYPES_FILE" ]; then
    echo "   Processing $TYPES_FILE..."
    
    # Check if StandardDomainConfig is used
    if grep -q "StandardDomainConfig" "$TYPES_FILE"; then
        echo "   ⚠️  Found StandardDomainConfig usage - needs manual migration"
        echo "   This crate uses custom extensions that need to be mapped to canonical"
    fi
fi

# Update config.rs to use canonical_master
CONFIG_FILE="$TARGET_CRATE/src/config.rs"
if [ -f "$CONFIG_FILE" ]; then
    echo "   Processing $CONFIG_FILE..."
    
    if grep -q "unified_config_master" "$CONFIG_FILE"; then
        echo "   ⚠️  Found unified_config_master usage - needs manual migration"
    fi
fi

echo ""
echo "📊 **MIGRATION SUMMARY**"
echo "========================================"
echo "   Status: Analysis complete"
echo "   Backup: $BACKUP_DIR"
echo ""
echo "⚠️  **MANUAL MIGRATION REQUIRED**"
echo ""
echo "The nestgate-network crate uses custom NetworkExtensions that need to be"
echo "mapped to CanonicalNetworkConfig's sub-modules. This requires manual migration."
echo ""
echo "**Recommended Approach**:"
echo "1. Review CanonicalNetworkConfig structure in canonical_master/domains/network/"
echo "2. Map NetworkExtensions fields to appropriate sub-modules"
echo "3. Update type alias to use CanonicalNetworkConfig"
echo "4. Add compatibility layer if needed"
echo ""
echo "**Files to Review**:"
echo "   - $TYPES_FILE"
echo "   - $CONFIG_FILE"
echo "   - $TARGET_CRATE/src/lib.rs"
echo ""
echo "========================================" 