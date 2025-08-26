#!/bin/bash
# Fix nested module references in canonical_modernization

set -e

echo "🔧 Fixing nested module references..."

# Function to fix a single file
fix_nested_refs() {
    local file="$1"
    local changed=false
    
    # Skip target directory
    if [[ "$file" == *"/target/"* ]]; then
        return
    fi
    
    # Create backup
    cp "$file" "$file.backup" 2>/dev/null || return
    
    # Fix nested service_types references
    if sed -i 's|canonical_modernization::service_types::|canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix nested health_types references
    if sed -i 's|canonical_modernization::health_types::|canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix nested unified_types references
    if sed -i 's|canonical_modernization::unified_types::|canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix nested unified_enums references
    if sed -i 's|canonical_modernization::unified_enums::|canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix other common nested references
    if sed -i 's|canonical_modernization::types::|canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    if sed -i 's|canonical_modernization::enums::|canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix constants nested references
    if sed -i 's|canonical_modernization::constants::|canonical_modernization::canonical_constants::|g' "$file"; then
        changed=true
    fi
    
    if [[ "$changed" == true ]]; then
        echo "✅ Fixed nested refs in $file"
    fi
    
    # Remove backup
    rm -f "$file.backup"
}

# Process all Rust files
echo "🔍 Processing Rust files..."
find . -name "*.rs" -type f | while read -r file; do
    fix_nested_refs "$file"
done

echo "🎯 Nested module reference fixes complete!" 