#!/bin/bash
# Automated Build Error Fixes - October 3, 2025
# This script fixes systematic issues found during build cleanup

set -e  # Exit on error

BACKUP_DIR="backups/automated-fix-$(date +%Y%m%d-%H%M%S)"
CODE_DIR="code/crates"

echo "🔧 Starting automated build fixes..."
echo "📁 Creating backup: $BACKUP_DIR"

# Create backup
mkdir -p "$BACKUP_DIR"
cp -r "$CODE_DIR" "$BACKUP_DIR/"

echo "✅ Backup created"

# Fix 1: Invalid error_message syntax
echo ""
echo "🔧 Fix 1: Removing invalid 'error_message:' named parameter..."
find "$CODE_DIR" -name "*.rs" -type f -exec sed -i 's/error_message: format!/\&format!/g' {} +
echo "✅ Fixed error_message syntax"

# Fix 2: Format string placeholders - actual_error_details
echo ""
echo "🔧 Fix 2: Fixing placeholder format strings..."
# This is trickier - we need to be careful not to break valid code
# For now, just report the count
PLACEHOLDER_COUNT=$(find "$CODE_DIR" -name "*.rs" -type f -exec grep -l '{"actual_error_details"}' {} + | wc -l)
echo "⚠️  Found $PLACEHOLDER_COUNT files with format string placeholders (needs manual review)"

# Fix 3: Const fn with to_string()
echo ""
echo "🔧 Fix 3: Removing inappropriate const fn markers (to_string calls)..."
# This is complex - we need to be selective
# For now, fix only default() and new() functions that call to_string
find "$CODE_DIR" -name "*.rs" -type f -exec sed -i 's/pub const fn default()/pub fn default()/g' {} +
find "$CODE_DIR" -name "*.rs" -type f -exec sed -i 's/pub const fn new(/pub fn new(/g' {} +
echo "✅ Fixed const fn markers for default() and new()"

# Fix 4: Const fn with format! macro
echo ""
echo "🔧 Fix 4: Removing const fn from functions using format!..."
# This requires more careful analysis - skip for now
echo "⚠️  Skipped - needs pattern analysis"

echo ""
echo "🎉 Automated fixes complete!"
echo "📊 Running build test..."

# Test the build
cargo build 2>&1 | grep "error\[E" | wc -l > /tmp/error_count.txt
ERROR_COUNT=$(cat /tmp/error_count.txt)

echo ""
echo "📊 Build error count: $ERROR_COUNT"
echo "💾 Backup location: $BACKUP_DIR"
echo ""
echo "Next steps:"
echo "1. Review the changes"
echo "2. Run: git diff code/crates"
echo "3. If satisfied, commit changes"
echo "4. If not, restore from: $BACKUP_DIR"

