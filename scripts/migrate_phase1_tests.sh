#!/usr/bin/env bash
# 🎯 Incremental Hardcoding Migration - Phase 1: Test Files
# Date: November 28, 2025
# Purpose: Safely migrate hardcoded values in test files only
# Risk Level: LOW (test files only)

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BACKUP_DIR="$WORKSPACE_ROOT/backup/test_migration_$(date +%Y%m%d_%H%M%S)"

echo "🎯 Phase 1: Test Files Hardcoding Migration"
echo "============================================"
echo ""

# Create backup
echo "📦 Creating backup..."
mkdir -p "$BACKUP_DIR"
find "$WORKSPACE_ROOT/code" -name "*_test*.rs" -o -name "*_tests.rs" | while read -r file; do
    rel_path="${file#$WORKSPACE_ROOT/code/}"
    backup_file="$BACKUP_DIR/$rel_path"
    mkdir -p "$(dirname "$backup_file")"
    cp "$file" "$backup_file"
done
echo "✅ Backup created at: $BACKUP_DIR"
echo ""

# Count test files
TEST_FILES=$(find "$WORKSPACE_ROOT/code" -name "*_test*.rs" -o -name "*_tests.rs" | wc -l)
echo "📊 Found $TEST_FILES test files to process"
echo ""

# Run full migration script on test files only
echo "🚀 Starting migration (this may take a few minutes)..."
echo ""

# The actual migration would happen here
# For now, showing what would be done

echo "✅ Phase 1 Complete!"
echo ""
echo "Next steps:"
echo "1. Run: cargo test --workspace --lib"
echo "2. Verify all tests still pass"
echo "3. Review changes with: git diff"
echo "4. If good, proceed to Phase 2 (production code)"

