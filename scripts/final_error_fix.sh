#!/bin/bash
echo "🎯 FINAL COMPREHENSIVE ERROR PATTERN FIX"

# Fix all malformed patterns in utils.rs by removing the extra trailing fields
sed -i 's/})), resource: None, retryable: true }/})/' code/crates/nestgate-core/src/utils.rs

# Check for any remaining malformed patterns across all files
echo "🔍 Scanning for any remaining malformed patterns..."
find code/crates/nestgate-core/src -name "*.rs" -exec grep -l "})), " {} \; 2>/dev/null | while read file; do
    echo "Fixing malformed patterns in $file..."
    sed -i 's/})), [^}]*}/})/' "$file"
done

echo "✅ All error patterns should now be fixed!"
