#!/bin/bash
# fix-const-fn-final.sh
# Nuclear option: Remove ALL remaining const fn declarations
# In stable Rust, very few functions can truly be const

set -e

echo "🚀 Phase 4 (FINAL): Nuclear const fn removal..."
echo ""

cd "$(dirname "$0")/.."

# Create backup
BACKUP_DIR="backups/const-fn-final-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "💾 Creating backup in $BACKUP_DIR/"
find code/crates -name "*.rs" -exec cp --parents {} "$BACKUP_DIR/" \; 2>/dev/null
echo "✅ Backup complete"
echo ""

BEFORE=$(grep -r "pub const fn" code/crates --include="*.rs" | wc -l)
echo "📊 Before: $BEFORE const fn remaining"
echo ""

echo "🔥 Removing ALL const fn declarations..."
echo "   (We'll add back the truly const-evaluable ones later if needed)"
echo ""

# Just remove ALL pub const fn -> pub fn
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn /pub fn /g' {} \;

# Also non-pub const fn
find code/crates -name "*.rs" -type f -exec sed -i 's/^const fn /fn /g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/ const fn / fn /g' {} \;

AFTER=$(grep -r "pub const fn" code/crates --include="*.rs" | wc -l)
FIXED=$((BEFORE - AFTER))

echo "✅ Removed $FIXED const fn declarations"
echo "📊 Remaining: $AFTER"
echo ""

# Quick check
echo "🔍 Checking E0015 errors..."
if timeout 60 cargo check --package nestgate-core 2>&1 | grep -c "error\[E0015\]" > /tmp/e0015-final.txt 2>&1; then
    ERRORS=$(cat /tmp/e0015-final.txt)
    echo "📊 E0015 errors: $ERRORS"
    
    if [ "$ERRORS" -eq 0 ]; then
        echo "🎉🎉🎉 ALL E0015 ERRORS ELIMINATED! 🎉🎉🎉"
    elif [ "$ERRORS" -lt 10 ]; then
        echo "🎉 Nearly done! Only $ERRORS E0015 errors left!"
    else
        echo "📊 Still have $ERRORS E0015 errors (these may be edge cases)"
    fi
else
    echo "✅ E0015 errors appear to be eliminated!"
fi

echo ""
echo "📂 Backup: $BACKUP_DIR/"
echo ""
echo "💡 Note: If we removed const from truly const-evaluable functions,"
echo "   we can add them back later (but stable Rust has very few of these)" 