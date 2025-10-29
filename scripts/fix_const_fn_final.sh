#!/bin/bash
# Final const fn cleanup - remove const from functions with non-const operations

echo "=== Final Const Fn Cleanup ===" 
echo "Starting at: $(date)"

# Create backup
BACKUP_DIR="backups/const-fn-final-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "Creating backup at: $BACKUP_DIR"
cp -r code/crates "$BACKUP_DIR/"

FIXED=0

# Find all files with const fn that have format!, to_string(), or Box::new()
echo "Finding files with problematic const fn..."

# Pattern 1: Remove const from functions using format!
for file in $(find code/crates -name "*.rs" -type f); do
    if grep -q "pub const fn" "$file" 2>/dev/null; then
        # Check if file has both const fn and format!/to_string/Box::new
        if grep -E "(format!|\.to_string\(\)|Box::new|tracing::|log::)" "$file" >/dev/null 2>&1; then
            echo "Processing: $file"
            
            # Remove const from pub const fn
            sed -i 's/pub const fn \([a-zA-Z_][a-zA-Z0-9_]*\)/pub fn \1/g' "$file" 2>/dev/null
            
            # Remove const from private const fn
            sed -i 's/^\(\s*\)const fn \([a-zA-Z_][a-zA-Z0-9_]*\)/\1fn \2/g' "$file" 2>/dev/null
            
            ((FIXED++))
        fi
    fi
done

echo ""
echo "=== Const Fn Cleanup Complete ==="
echo "Files processed: $FIXED"
echo "Backup saved to: $BACKUP_DIR"
echo ""
echo "Next: Run 'cargo build 2>&1 | grep -c \"^error\"' to verify"
echo "Finished at: $(date)"

