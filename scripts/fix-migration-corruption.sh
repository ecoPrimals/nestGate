#!/bin/bash

# **MIGRATION CORRUPTION CLEANUP SCRIPT**
# 
# This script fixes files that were corrupted by the async_trait migration script
# by removing log messages that were incorrectly inserted into the code.

set -euo pipefail

echo "🔧 **FIXING MIGRATION CORRUPTION**"
echo "=================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CODE_DIR="$PROJECT_ROOT/code"

# Find all files with corruption markers
CORRUPTED_FILES=$(find "$CODE_DIR" -name "*.rs" -exec grep -l "⚠️.*Removing async_trait annotation\|🔧.*Converting trait to native async\|🔄.*Converted method" {} \; 2>/dev/null || true)

if [ -z "$CORRUPTED_FILES" ]; then
    echo "✅ No corrupted files found"
    exit 0
fi

echo "Found corrupted files:"
echo "$CORRUPTED_FILES" | while read -r file; do
    if [ -n "$file" ]; then
        echo "  📁 $file"
    fi
done
echo ""

echo "🧹 Cleaning corrupted files..."

# Clean up each corrupted file
echo "$CORRUPTED_FILES" | while read -r file; do
    if [ -n "$file" ] && [ -f "$file" ]; then
        echo "  🔄 Fixing: $file"
        
        # Create backup
        cp "$file" "${file}.corruption_backup"
        
        # Remove corruption markers
        sed -i '/⚠️.*Removing async_trait annotation/d' "$file"
        sed -i '/🔧.*Converting trait to native async/d' "$file"
        sed -i '/🔄.*Converted method/d' "$file"
        
        # Fix any malformed async method definitions
        # Look for patterns like: fn method_name(...) -> impl Future<...> + Send;
        # followed by stray lines that should be inside the method
        
        # This is a complex fix, so we'll do basic cleanup
        # and let the user handle specific syntax errors
        
        echo "  ✅ Cleaned: $file"
    fi
done

echo ""
echo "🎯 Running compilation check..."

cd "$PROJECT_ROOT"
if cargo check --workspace --quiet 2>/dev/null; then
    echo "✅ All files compile successfully!"
    
    # Remove backup files since cleanup was successful
    find "$CODE_DIR" -name "*.corruption_backup" -delete
    echo "🧹 Removed backup files"
else
    echo "⚠️  Some compilation issues remain"
    echo "📋 Manual review needed for remaining syntax errors"
    echo "💾 Backup files preserved for manual recovery if needed"
fi

echo ""
echo "✅ **CORRUPTION CLEANUP COMPLETE**" 