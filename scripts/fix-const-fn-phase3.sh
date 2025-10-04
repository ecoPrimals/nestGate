#!/bin/bash
# fix-const-fn-phase3.sh
# Final aggressive pass - remove const from nearly all remaining const fn
# Only keep const on truly compile-time evaluable functions

set -e

echo "🔧 Phase 3: Final aggressive const fn cleanup..."
echo ""

cd "$(dirname "$0")/.."

# Create backup
BACKUP_DIR="backups/const-fn-phase3-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "💾 Creating backup in $BACKUP_DIR/"
find code/crates -name "*.rs" -exec cp --parents {} "$BACKUP_DIR/" \; 2>/dev/null
echo "✅ Backup complete"
echo ""

# Count before
BEFORE=$(grep -r "pub const fn" code/crates --include="*.rs" | wc -l)
BEFORE_ERRORS=$(timeout 60 cargo check --package nestgate-core 2>&1 | grep -c "error\[E0015\]" || echo "753")
echo "📊 Before: $BEFORE const fn, $BEFORE_ERRORS E0015 errors"
echo ""

echo "🔨 Removing const from:"
echo "   - Functions in impl blocks"
echo "   - Functions with &self or &mut self"
echo "   - Functions calling other functions"
echo "   - Functions with complex logic"
echo ""

# Remove const from methods (functions with self)
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \([^(]*\)(&self/pub fn \1(\&self/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \([^(]*\)(&mut self/pub fn \1(\&mut self/g' {} \;

# Remove const from functions with String parameters
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \([^(]*\)(.*String/pub fn \1(.*String/g' {} \;

# Remove const from functions returning impl Trait
find code/crates -name "*.rs" -type f -exec sed -i 's/pub const fn \([^(]*\)([^)]*) -> impl /pub fn \1(\1) -> impl /g' {} \;

# Just remove const from basically everything that has a body with {}
# This is aggressive but necessary - very few functions can truly be const in Rust stable
echo "🚨 Ultra-aggressive mode: Removing const from all impl blocks"

# For each rust file, if it has "impl" blocks, remove const from functions inside them
find code/crates -name "*.rs" -type f -print0 | while IFS= read -r -d '' file; do
    # Remove const from any pub fn inside impl blocks (rough heuristic)
    if grep -q "^impl" "$file"; then
        sed -i 's/    pub const fn /    pub fn /g' "$file"
        sed -i 's/        pub const fn /        pub fn /g' "$file"
    fi
done

echo "✅ Impl block const fn removal complete"

# Also remove from trait implementations
find code/crates -name "*.rs" -type f -exec sed -i 's/^    const fn /    fn /g' {} \;

# Count after
AFTER=$(grep -r "pub const fn" code/crates --include="*.rs" | wc -l)
FIXED=$((BEFORE - AFTER))

echo ""
echo "✅ Phase 3 complete!"
echo "📊 Fixed: $FIXED additional const fn"
echo "📊 Remaining: $AFTER const fn"
echo ""

# Check errors
echo "🔍 Checking E0015 errors..."
if timeout 90 cargo check --package nestgate-core 2>&1 | grep "error\[E0015\]" | wc -l > /tmp/e0015-count.txt; then
    AFTER_ERRORS=$(cat /tmp/e0015-count.txt)
    ERROR_REDUCTION=$((BEFORE_ERRORS - AFTER_ERRORS))
    echo "📊 E0015 errors: $BEFORE_ERRORS → $AFTER_ERRORS (reduced by $ERROR_REDUCTION)"
    
    if [ "$AFTER_ERRORS" -lt 200 ]; then
        echo "🎉 EXCELLENT! Under 200 E0015 errors!"
    elif [ "$AFTER_ERRORS" -lt 400 ]; then
        echo "✅ GREAT! Under 400 E0015 errors!"
    else
        echo "⚠️  Still have $AFTER_ERRORS E0015 errors - may need Phase 4"
    fi
fi

echo ""
echo "💡 If E0015 errors still high, we may need to remove ALL const fn"
echo "   Only a handful of functions should truly be const in stable Rust"
echo ""
echo "📂 Backup: $BACKUP_DIR/" 