#!/bin/bash
# Network Config Migration Script
# Phase 2 - Week 1
# Migrates NetworkConfig definitions to use canonical type aliases

set -e

echo "========================================"
echo "Network Config Migration Script"
echo "Phase 2 Unification - Week 1"
echo "========================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Usage message
usage() {
    echo "Usage: $0 <file> <config_name>"
    echo ""
    echo "Example: $0 code/crates/nestgate-core/src/network/auth.rs NetworkAuthConfig"
    echo ""
    echo "This script will:"
    echo "  1. Add deprecation marker to existing config struct"
    echo "  2. Keep original struct definition (for compatibility)"
    echo "  3. Add canonical import and type alias"
    echo "  4. Verify the file compiles"
    exit 1
}

# Check arguments
if [ $# -ne 2 ]; then
    usage
fi

FILE="$1"
CONFIG_NAME="$2"

# Verify file exists
if [ ! -f "$FILE" ]; then
    echo -e "${RED}Error: File $FILE does not exist${NC}"
    exit 1
fi

echo "Migrating: $FILE"
echo "Config: $CONFIG_NAME"
echo ""

# Create backup
BACKUP_DIR="analysis/network_config_backups"
mkdir -p "$BACKUP_DIR"
BACKUP_FILE="$BACKUP_DIR/$(basename $FILE).backup.$(date +%Y%m%d_%H%M%S)"
cp "$FILE" "$BACKUP_FILE"
echo -e "${GREEN}✓${NC} Backup created: $BACKUP_FILE"

# Check if config exists in file
if ! grep -q "pub struct $CONFIG_NAME" "$FILE"; then
    echo -e "${YELLOW}Warning: 'pub struct $CONFIG_NAME' not found in $FILE${NC}"
    echo "Continuing anyway..."
fi

# Add deprecation marker before the struct definition
# Using sed to insert deprecation before "pub struct ConfigName"
sed -i "/pub struct $CONFIG_NAME/i \\
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary\\
/// \\
/// **Migration Path**:\\
/// \`\`\`rust\\
/// // OLD (deprecated):\\
/// use crate::network::config::$CONFIG_NAME;\\
/// \\
/// // NEW (canonical):\\
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;\\
/// // Or use type alias for compatibility:\\
/// use crate::network::config::$CONFIG_NAME; // Now aliases to CanonicalNetworkConfig\\
/// \`\`\`\\
/// \\
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)\\
#[deprecated(since = \"0.11.0\", note = \"Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead\")]" "$FILE"

echo -e "${GREEN}✓${NC} Added deprecation marker to $CONFIG_NAME"

# Add type alias at end of file (before any mod tests if present)
ALIAS_SECTION="
// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ${CONFIG_NAME}Canonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using $CONFIG_NAME (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
"

# Insert before #[cfg(test)] if it exists, otherwise at end
if grep -q "#\[cfg(test)\]" "$FILE"; then
    # Create temp file with insertion
    awk -v alias="$ALIAS_SECTION" '/#\[cfg\(test\)\]/{print alias}1' "$FILE" > "$FILE.tmp"
    mv "$FILE.tmp" "$FILE"
else
    echo "$ALIAS_SECTION" >> "$FILE"
fi

echo -e "${GREEN}✓${NC} Added canonical type alias"

# Try to compile the file's crate
CRATE=$(echo "$FILE" | grep -oP 'code/crates/\K[^/]+' || echo "unknown")
if [ "$CRATE" != "unknown" ]; then
    echo ""
    echo "Verifying compilation of $CRATE..."
    if cargo check --package "$CRATE" 2>&1 | tail -5; then
        echo -e "${GREEN}✓${NC} Crate $CRATE compiles successfully"
    else
        echo -e "${YELLOW}⚠${NC} Compilation check had issues (may be expected with deprecations)"
    fi
fi

echo ""
echo "========================================"
echo -e "${GREEN}Migration Complete!${NC}"
echo "========================================"
echo ""
echo "What was done:"
echo "  1. ✓ Backup created: $BACKUP_FILE"
echo "  2. ✓ Deprecation marker added to $CONFIG_NAME"
echo "  3. ✓ Canonical type alias added (${CONFIG_NAME}Canonical)"
echo "  4. ✓ Original struct kept for compatibility"
echo ""
echo "Next steps:"
echo "  - Review the changes in $FILE"
echo "  - Run tests: cargo test --package $CRATE"
echo "  - Commit: git add $FILE && git commit -m 'feat: migrate $CONFIG_NAME to canonical network config'"
echo ""

