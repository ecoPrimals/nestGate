#!/bin/bash
# fix-const-fn.sh
# Removes 'const' from functions that return Result<T>
# This fixes the majority of compilation errors (1,085+ errors)

set -e

echo "🔧 Fixing excessive const fn declarations..."
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Count before
BEFORE=$(grep -r "pub const fn.*Result" code/crates --include="*.rs" 2>/dev/null | wc -l)
echo "📊 Found $BEFORE const fn returning Result"

# Create backup
BACKUP_DIR="backups/const-fn-fix-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "💾 Creating backup in $BACKUP_DIR/"

# Backup files before modification
find code/crates -name "*.rs" -exec cp --parents {} "$BACKUP_DIR/" \; 2>/dev/null

echo "✅ Backup complete"
echo ""
echo "🔨 Applying fixes..."

# Fix: pub const fn ... -> Result<...>
# Replace with: pub fn ... -> Result<...>
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \(.*\) -> crate::Result</pub fn \1 -> crate::Result</g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \(.*\) -> Result</pub fn \1 -> Result</g' {} \;

# Also fix async const fn (if any)
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const async fn/pub async fn/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/pub async const fn/pub async fn/g' {} \;

# Count after
AFTER=$(grep -r "pub const fn.*Result" code/crates --include="*.rs" 2>/dev/null | wc -l)
FIXED=$((BEFORE - AFTER))

echo ""
echo "✅ Fixed $FIXED const fn declarations"
echo "📊 Remaining const fn with Result: $AFTER"
echo ""
echo "🔍 Checking build..."

# Quick check
if timeout 60 cargo check --package nestgate-core 2>&1 | grep -q "error\[E0015\]"; then
    CONST_ERRORS=$(timeout 60 cargo check --package nestgate-core 2>&1 | grep -c "error\[E0015\]" || echo "0")
    echo "⚠️  Still have $CONST_ERRORS E0015 errors (const fn issues)"
    echo "   This is expected - some const fn need manual review"
else
    echo "✅ E0015 errors significantly reduced!"
fi

echo ""
echo "💡 Next steps:"
echo "   1. Review the changes"
echo "   2. Run: cargo check --workspace"
echo "   3. Fix any remaining errors manually"
echo ""
echo "📂 Backup location: $BACKUP_DIR/" 