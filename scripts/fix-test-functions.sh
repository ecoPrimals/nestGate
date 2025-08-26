#!/bin/bash

# Script to fix test function signatures that use ? operator
# This will update functions to return Result<(), Box<dyn std::error::Error>>

echo "🔧 Fixing test function signatures..."

# Find all .rs files in the codebase
find . -name "*.rs" -type f | while read -r file; do
    echo "Processing: $file"
    
    # Fix #[test] functions that use ? operator
    sed -i 's/#\[test\]\s*\n\s*fn \([^(]*\)() {/#[test]\nfn \1() -> Result<(), Box<dyn std::error::Error>> {/g' "$file"
    
    # Fix #[tokio::test] functions that use ? operator  
    sed -i 's/#\[tokio::test\]\s*\n\s*async fn \([^(]*\)() {/#[tokio::test]\nasync fn \1() -> Result<(), Box<dyn std::error::Error>> {/g' "$file"
    
    # Add Ok(()) returns to functions that end with just }
    # This is more complex and should be done carefully
done

echo "✅ Test function signatures updated"
echo "⚠️  Note: You may need to manually add Ok(()) returns to function ends" 