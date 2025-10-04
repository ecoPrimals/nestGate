#!/bin/bash
# fix-remaining-const-fn.sh  
# Aggressively removes 'const' from functions that can't be const

set -e

echo "🔧 Phase 2: Fixing remaining const fn issues..."
echo ""

cd "$(dirname "$0")/.."

# Create backup
BACKUP_DIR="backups/const-fn-phase2-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "💾 Creating backup in $BACKUP_DIR/"
find code/crates -name "*.rs" -exec cp --parents {} "$BACKUP_DIR/" \; 2>/dev/null
echo "✅ Backup complete"
echo ""

# Count before
BEFORE=$(grep -r "pub const fn" code/crates --include="*.rs" | wc -l)
echo "📊 Total const fn before: $BEFORE"
echo ""

echo "🔨 Removing const from functions that:"
echo "   - Use Option/String/Vec types"
echo "   - Call .clone(), .to_string(), new()"
echo "   - Use Arc, Mutex, or other non-const types"
echo ""

# Remove const from fn that use common non-const patterns
# Functions with Option<String>
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \(.*Option<String>\)/pub fn \1/g' {} \;

# Functions with Vec
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \(.*Vec<\)/pub fn \1/g' {} \;

# Functions with Arc
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \(.*Arc<\)/pub fn \1/g' {} \;

# Functions with Box
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \(.*Box<\)/pub fn \1/g' {} \;

# Functions with HashMap
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \(.*HashMap\)/pub fn \1/g' {} \;

# Functions named "new" (constructors usually can't be const)
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn new(/pub fn new(/g' {} \;

# Functions named "create"
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn create/pub fn create/g' {} \;

# Functions named "build"  
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn build(/pub fn build(/g' {} \;

# Functions named "validate" (already did some of these)
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn validate(/pub fn validate(/g' {} \;

# Count after
AFTER=$(grep -r "pub const fn" code/crates --include="*.rs" | wc -l)
FIXED=$((BEFORE - AFTER))

echo ""
echo "✅ Phase 2 complete!"
echo "📊 Fixed: $FIXED additional const fn declarations"
echo "📊 Remaining: $AFTER const fn"
echo ""
echo "🔍 Checking errors..."

# Check error count
if timeout 90 cargo check --package nestgate-core 2>&1 | grep "error\[E0015\]" | wc -l > /tmp/const-errors.txt; then
    ERRORS=$(cat /tmp/const-errors.txt)
    echo "📊 E0015 errors remaining: $ERRORS"
    
    if [ "$ERRORS" -lt 200 ]; then
        echo "🎉 Great progress! Under 200 errors remaining"
    elif [ "$ERRORS" -lt 500 ]; then
        echo "✅ Good progress! Reduced significantly"
    fi
fi

echo ""
echo "📂 Backup: $BACKUP_DIR/" 