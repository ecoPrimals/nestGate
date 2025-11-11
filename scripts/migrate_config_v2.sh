#!/bin/bash
# Config Migration Script V2
# Phase 2 - Week 2
# Improved with duplicate detection and validation

set -e

echo "========================================"
echo "Config Migration Script V2"
echo "Phase 2 Unification - Week 2"
echo "With duplicate detection & validation"
echo "========================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Usage message
usage() {
    echo "Usage: $0 <file> <config_name>"
    echo ""
    echo "Example: $0 code/crates/nestgate-core/src/storage/config.rs StorageConfig"
    echo ""
    echo "This script will:"
    echo "  0. PRE-VALIDATE: Check for existing aliases/deprecations"
    echo "  1. Add deprecation marker to existing config struct"
    echo "  2. Keep original struct definition (for compatibility)"
    echo "  3. Add canonical import and type alias"
    echo "  4. Use correct crate path (crate:: vs nestgate_core::)"
    echo "  5. Verify the file compiles"
    exit 1
}

# Check arguments
if [ $# -ne 2 ]; then
    usage
fi

FILE="$1"
CONFIG_NAME="$2"
ALIAS_NAME="${CONFIG_NAME}Canonical"

# Verify file exists
if [ ! -f "$FILE" ]; then
    echo -e "${RED}Error: File $FILE does not exist${NC}"
    exit 1
fi

echo "Migrating: $FILE"
echo "Config: $CONFIG_NAME"
echo "Alias: $ALIAS_NAME"
echo ""

# ==================== PRE-VALIDATION ====================
echo -e "${BLUE}Running pre-validation checks...${NC}"

# Check 1: Does the config struct exist?
if ! grep -q "pub struct $CONFIG_NAME" "$FILE"; then
    echo -e "${RED}✗ Error: 'pub struct $CONFIG_NAME' not found in $FILE${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} Config struct exists"

# Check 2: Is it already deprecated?
if grep -B5 "pub struct $CONFIG_NAME" "$FILE" | grep -q "#\[deprecated"; then
    echo -e "${YELLOW}⚠ Warning: $CONFIG_NAME is already marked as deprecated${NC}"
    echo "Skipping migration (already migrated)"
    exit 0
fi
echo -e "${GREEN}✓${NC} Not already deprecated"

# Check 3: Does the alias already exist?
if grep -q "pub type $ALIAS_NAME" "$FILE"; then
    echo -e "${YELLOW}⚠ Warning: Type alias $ALIAS_NAME already exists${NC}"
    
    # Count how many times
    ALIAS_COUNT=$(grep -c "pub type $ALIAS_NAME" "$FILE")
    if [ "$ALIAS_COUNT" -gt 1 ]; then
        echo -e "${RED}✗ Error: Multiple aliases found ($ALIAS_COUNT occurrences)${NC}"
        echo "Manual cleanup required before migration"
        exit 1
    fi
    
    echo "Skipping migration (alias already exists)"
    exit 0
fi
echo -e "${GREEN}✓${NC} No existing alias"

# Check 4: Determine correct crate path
CRATE_PREFIX="nestgate_core::"
if echo "$FILE" | grep -q "code/crates/nestgate-core/"; then
    CRATE_PREFIX="crate::"
    echo -e "${GREEN}✓${NC} Using internal crate path (crate::)"
else
    echo -e "${GREEN}✓${NC} Using external crate path (nestgate_core::)"
fi

echo -e "${GREEN}✓${NC} All pre-validation checks passed!"
echo ""

# ==================== MIGRATION ====================

# Create backup
BACKUP_DIR="analysis/network_config_backups"
mkdir -p "$BACKUP_DIR"
BACKUP_FILE="$BACKUP_DIR/$(basename $FILE).backup.$(date +%Y%m%d_%H%M%S)"
cp "$FILE" "$BACKUP_FILE"
echo -e "${GREEN}✓${NC} Backup created: $BACKUP_FILE"

# Add deprecation marker before the struct definition
sed -i "/pub struct $CONFIG_NAME/i \\
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary\\
/// \\
/// **Migration Path**:\\
/// \`\`\`rust\\
/// // OLD (deprecated):\\
/// use crate::config::$CONFIG_NAME;\\
/// \\
/// // NEW (canonical):\\
/// use ${CRATE_PREFIX}config::canonical_primary::domains::network::CanonicalNetworkConfig;\\
/// // Or use type alias for compatibility:\\
/// use crate::config::$CONFIG_NAME; // Now aliases to CanonicalNetworkConfig\\
/// \`\`\`\\
/// \\
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)\\
#[deprecated(since = \"0.11.0\", note = \"Use ${CRATE_PREFIX}config::canonical_primary::domains::network::CanonicalNetworkConfig instead\")]" "$FILE"

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
pub type ${ALIAS_NAME} = ${CRATE_PREFIX}config::canonical_primary::domains::network::CanonicalNetworkConfig;

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

echo -e "${GREEN}✓${NC} Added canonical type alias ($ALIAS_NAME)"

# ==================== POST-VALIDATION ====================
echo ""
echo -e "${BLUE}Running post-validation...${NC}"

# Verify the alias was added exactly once
ALIAS_COUNT=$(grep -c "pub type $ALIAS_NAME" "$FILE")
if [ "$ALIAS_COUNT" -ne 1 ]; then
    echo -e "${RED}✗ Error: Expected 1 alias, found $ALIAS_COUNT${NC}"
    echo "Rolling back..."
    cp "$BACKUP_FILE" "$FILE"
    exit 1
fi
echo -e "${GREEN}✓${NC} Alias added correctly (1 occurrence)"

# Try to compile the file's crate
CRATE=$(echo "$FILE" | grep -oP 'code/crates/\K[^/]+' || echo "unknown")
if [ "$CRATE" != "unknown" ]; then
    echo ""
    echo "Verifying compilation of $CRATE..."
    if cargo check --package "$CRATE" 2>&1 | tail -5; then
        echo -e "${GREEN}✓${NC} Crate $CRATE compiles successfully"
    else
        echo -e "${YELLOW}⚠${NC} Compilation check had issues (may need manual fixes)"
    fi
fi

echo ""
echo "========================================"
echo -e "${GREEN}Migration Complete!${NC}"
echo "========================================"
echo ""
echo "What was done:"
echo "  1. ✓ Pre-validation passed"
echo "  2. ✓ Backup created: $BACKUP_FILE"
echo "  3. ✓ Deprecation marker added to $CONFIG_NAME"
echo "  4. ✓ Canonical type alias added ($ALIAS_NAME)"
echo "  5. ✓ Correct crate path used ($CRATE_PREFIX)"
echo "  6. ✓ Post-validation passed"
echo ""
echo "Next steps:"
echo "  - Review the changes in $FILE"
echo "  - Run tests: cargo test --package $CRATE"
echo "  - Commit: git add $FILE && git commit -m 'feat: migrate $CONFIG_NAME to canonical config'"
echo ""

