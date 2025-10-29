#!/bin/bash
# Aggressive Const Fn Fixes - October 3, 2025
# Remove const fn markers from functions that can't be const

set -e

echo "🔧 Phase 2: Aggressive const fn cleanup..."

# Backup
BACKUP_DIR="backups/const-fn-fix-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
cp -r code/crates "$BACKUP_DIR/"
echo "✅ Backup: $BACKUP_DIR"

CODE_DIR="code/crates"

# Remove const fn from any function that:
# 1. Contains format!
# 2. Contains .to_string()
# 3. Contains Box::new
# 4. Contains String::from

echo "🔧 Removing const fn from functions with format! macro..."
find "$CODE_DIR" -name "*.rs" -type f | while read file; do
    # Check if file contains both "pub const fn" and "format!"
    if grep -q "pub const fn" "$file" && grep -q "format!" "$file"; then
        # For each const fn in the file, check if it contains format!
        # This is a simplified approach - remove const from all functions in files that use format!
        sed -i 's/pub const fn /pub fn /g' "$file"
    fi
done
echo "✅ Fixed format! functions"

echo "🔧 Removing const fn from functions with .to_string()..."
find "$CODE_DIR" -name "*.rs" -type f | while read file; do
    if grep -q "pub const fn" "$file" && grep -q "\.to_string()" "$file"; then
        sed -i 's/pub const fn /pub fn /g' "$file"
    fi
done
echo "✅ Fixed to_string() functions"

echo "🔧 Removing const fn from functions with Box::new..."
find "$CODE_DIR" -name "*.rs" -type f | while read file; do
    if grep -q "pub const fn" "$file" && grep -q "Box::new" "$file"; then
        sed -i 's/pub const fn /pub fn /g' "$file"
    fi
done
echo "✅ Fixed Box::new functions"

echo "🔧 Removing const fn from functions with tracing macros..."
find "$CODE_DIR" -name "*.rs" -type f | while read file; do
    if grep -q "pub const fn" "$file" && (grep -q "info!" "$file" || grep -q "debug!" "$file" || grep -q "warn!" "$file"); then
        sed -i 's/pub const fn /pub fn /g' "$file"
    fi
done
echo "✅ Fixed tracing macro functions"

echo ""
echo "🎉 Aggressive const fn cleanup complete!"
echo "📊 Testing build..."

cargo build 2>&1 | grep "^error\[E" | wc -l > /tmp/error_count2.txt
ERROR_COUNT=$(cat /tmp/error_count2.txt)

echo "📊 Build error count: $ERROR_COUNT"
echo "💾 Backup: $BACKUP_DIR"

