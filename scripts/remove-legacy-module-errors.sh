#!/bin/bash
# remove-legacy-module-errors.sh
# Automatically removes LegacyModuleError boilerplate instances

set -euo pipefail

echo "🧹 **REMOVING LEGACY MODULE ERROR BOILERPLATE**"
echo "================================================"

cd "$(dirname "$0")/.."

# Find all files with LegacyModuleError
echo ""
echo "📊 Finding LegacyModuleError instances..."
FILES_WITH_LEGACY=$(grep -r "pub enum LegacyModuleError" code/crates/ --include="*.rs" -l 2>/dev/null || true)

if [ -z "$FILES_WITH_LEGACY" ]; then
    echo "   ✅ No LegacyModuleError instances found!"
    exit 0
fi

FILE_COUNT=$(echo "$FILES_WITH_LEGACY" | wc -l | tr -d ' \n')
echo "   Found $FILE_COUNT files with LegacyModuleError"
echo ""

# Create backup
BACKUP_DIR="backups/legacy-error-removal-$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "📦 Creating backup in $BACKUP_DIR..."

# Process each file
REMOVED_COUNT=0
FAILED_COUNT=0

while IFS= read -r file; do
    if [ -z "$file" ]; then
        continue
    fi
    
    echo "   Processing: $file"
    
    # Create backup
    cp "$file" "$BACKUP_DIR/$(basename "$file").backup"
    
    # Remove the LegacyModuleError enum definition
    # Pattern matches: pub enum LegacyModuleError { Unknown(String), }
    if sed -i '/pub enum LegacyModuleError {/,/^}/d' "$file" 2>/dev/null; then
        # Also remove any Result type alias using LegacyModuleError
        sed -i '/type.*Result.*LegacyModuleError/d' "$file" 2>/dev/null || true
        
        # Remove #[deprecated] marker if it's alone on a line before the enum
        sed -i '/#\[deprecated.*LegacyModuleError/d' "$file" 2>/dev/null || true
        
        REMOVED_COUNT=$((REMOVED_COUNT + 1))
        echo "      ✅ Removed LegacyModuleError from $file"
    else
        FAILED_COUNT=$((FAILED_COUNT + 1))
        echo "      ❌ Failed to process $file"
    fi
done <<< "$FILES_WITH_LEGACY"

echo ""
echo "================================================"
echo "📊 **REMOVAL SUMMARY**"
echo "================================================"
echo "   Files processed: $FILE_COUNT"
echo "   Successfully removed: $REMOVED_COUNT"
echo "   Failed: $FAILED_COUNT"
echo "   Backup location: $BACKUP_DIR"
echo ""

# Run cargo check to see if we broke anything
echo "🔨 Running cargo check to verify changes..."
if cargo check --workspace --quiet 2>&1 | head -20; then
    echo "   ✅ Cargo check passed!"
else
    echo "   ⚠️  Cargo check has issues. Review the output above."
    echo "   If needed, restore from backup: $BACKUP_DIR"
fi

echo ""
echo "================================================"
if [ "$REMOVED_COUNT" -gt 0 ]; then
    echo "✅ **LegacyModuleError removal complete!**"
    echo ""
    echo "Next steps:"
    echo "1. Review the cargo check output above"
    echo "2. Run: ./scripts/validation/validate-error-unification.sh"
    echo "3. Commit changes if validation passes"
    echo "4. Backup is in: $BACKUP_DIR"
else
    echo "ℹ️  No LegacyModuleError instances to remove"
fi
echo "================================================" 