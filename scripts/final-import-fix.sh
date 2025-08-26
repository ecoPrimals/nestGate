#!/bin/bash
# Final Import Fix Script - Complete canonical modernization

set -e

echo "🔧 Final Import Path Fixes"

# Function to fix a single file
fix_file() {
    local file="$1"
    local changed=false
    
    # Skip target directory
    if [[ "$file" == *"/target/"* ]]; then
        return
    fi
    
    # Create backup
    cp "$file" "$file.backup" 2>/dev/null || return
    
    # Fix unified_types references
    if sed -i 's|crate::unified_types::|crate::canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix unified_enums references
    if sed -i 's|crate::unified_enums::|crate::canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix specific nested paths
    if sed -i 's|crate::unified_enums::health_types::|crate::canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    if sed -i 's|crate::unified_enums::service_types::|crate::canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix use statements
    if sed -i 's|use.*unified_types::|use crate::canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    if sed -i 's|use.*unified_enums::|use crate::canonical_modernization::|g' "$file"; then
        changed=true
    fi
    
    # Fix constants references
    if sed -i 's|crate::constants::|crate::canonical_modernization::canonical_constants::|g' "$file"; then
        changed=true
    fi
    
    # Fix diagnostics references
    if sed -i 's|crate::diagnostics::|crate::service_discovery::|g' "$file"; then
        changed=true
    fi
    
    if [[ "$changed" == true ]]; then
        echo "✅ Fixed $file"
    fi
    
    # Remove backup
    rm -f "$file.backup"
}

# Process all Rust files
echo "🔍 Processing Rust files..."
find . -name "*.rs" -type f | while read -r file; do
    fix_file "$file"
done

echo "🎯 Import fixes complete!" 