#!/bin/bash
# Wire Up Test Module Script
# Adds module imports for orphaned test files

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if [ -z "$1" ]; then
    echo "Usage: $0 <crate-name> [--dry-run]"
    echo "Example: $0 nestgate-api"
    echo "Example: $0 nestgate-core --dry-run"
    exit 1
fi

CRATE_NAME="$1"
DRY_RUN=false

if [ "$2" = "--dry-run" ]; then
    DRY_RUN=true
    echo "🔍 DRY RUN MODE - No changes will be made"
fi

CRATE_DIR="code/crates/$CRATE_NAME"

if [ ! -d "$CRATE_DIR" ]; then
    echo "❌ Crate directory not found: $CRATE_DIR"
    exit 1
fi

echo "🔧 Wiring up tests for $CRATE_NAME"
echo "📁 Crate directory: $CRATE_DIR"
echo ""

# Get orphaned tests for this crate
ORPHANED_FILE="test-wiring-audit/orphaned_tests.txt"
if [ ! -f "$ORPHANED_FILE" ]; then
    echo "❌ Orphaned tests file not found. Run find_orphaned_tests.sh first."
    exit 1
fi

# Filter for this crate
grep "^$CRATE_DIR/" "$ORPHANED_FILE" > "/tmp/${CRATE_NAME}_orphaned.txt" 2>/dev/null || touch "/tmp/${CRATE_NAME}_orphaned.txt"

ORPHAN_COUNT=$(wc -l < "/tmp/${CRATE_NAME}_orphaned.txt")

if [ "$ORPHAN_COUNT" -eq 0 ]; then
    echo "✅ No orphaned test files found for $CRATE_NAME"
    exit 0
fi

echo "📋 Found $ORPHAN_COUNT orphaned test files"
echo ""

# Process each orphaned file
WIRED_COUNT=0
SKIPPED_COUNT=0

while IFS= read -r test_file; do
    if [ -z "$test_file" ]; then
        continue
    fi
    
    echo "Processing: $test_file"
    
    # Get module name and parent directory
    module_name=$(basename "$test_file" .rs)
    parent_dir=$(dirname "$test_file")
    
    # Determine which file to modify (mod.rs or lib.rs)
    target_file=""
    if [ -f "$parent_dir/mod.rs" ]; then
        target_file="$parent_dir/mod.rs"
    elif [ -f "$parent_dir/lib.rs" ]; then
        target_file="$parent_dir/lib.rs"
    else
        # Try parent's parent lib.rs
        parent_parent=$(dirname "$parent_dir")
        if [ -f "$parent_parent/lib.rs" ]; then
            target_file="$parent_parent/lib.rs"
        fi
    fi
    
    if [ -z "$target_file" ]; then
        echo "  ⚠️  Could not find mod.rs or lib.rs for $test_file"
        SKIPPED_COUNT=$((SKIPPED_COUNT + 1))
        continue
    fi
    
    # Check if already imported
    if grep -q "mod $module_name" "$target_file" 2>/dev/null; then
        echo "  ℹ️  Already imported in $(basename "$target_file")"
        continue
    fi
    
    # Add the module import
    import_line="#[cfg(test)]\nmod $module_name;"
    
    if [ "$DRY_RUN" = true ]; then
        echo "  🔍 Would add to $target_file:"
        echo "      $import_line"
        WIRED_COUNT=$((WIRED_COUNT + 1))
    else
        # Add import at the end of the file
        echo "" >> "$target_file"
        echo "#[cfg(test)]" >> "$target_file"
        echo "mod $module_name;" >> "$target_file"
        echo "  ✅ Added to $(basename "$target_file")"
        WIRED_COUNT=$((WIRED_COUNT + 1))
    fi
    
    echo ""
done < "/tmp/${CRATE_NAME}_orphaned.txt"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Summary for $CRATE_NAME:"
echo "  ✅ Wired: $WIRED_COUNT"
echo "  ⚠️  Skipped: $SKIPPED_COUNT"
echo ""

if [ "$DRY_RUN" = false ] && [ "$WIRED_COUNT" -gt 0 ]; then
    echo "🧪 Running cargo check..."
    if cargo check --package "$CRATE_NAME" --lib 2>&1 | tail -20; then
        echo "✅ Cargo check passed!"
    else
        echo "⚠️  Cargo check found issues - review and fix"
    fi
    echo ""
    
    echo "🧪 Running tests..."
    if cargo test --package "$CRATE_NAME" --lib --no-fail-fast 2>&1 | tail -30; then
        echo "✅ Tests complete!"
    else
        echo "⚠️  Some tests may have failed - review output"
    fi
fi

rm "/tmp/${CRATE_NAME}_orphaned.txt"

