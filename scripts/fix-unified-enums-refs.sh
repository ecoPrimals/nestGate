#!/bin/bash
# Fix unified_enums references to use canonical_modernization

set -e

echo "🔄 Fixing unified_enums references..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

TOTAL_FILES=0
FIXED_FILES=0

# Function to fix unified_enums imports in a file
fix_unified_enums_file() {
    local file="$1"
    local changed=false
    
    # Skip target directory
    if [[ "$file" == *"/target/"* ]]; then
        return
    fi
    
    # Check if file contains unified_enums references
    if grep -q "crate::unified_enums::" "$file" 2>/dev/null || grep -q "use.*unified_enums::" "$file" 2>/dev/null; then
        echo -e "${BLUE}ℹ️  Fixing $file${NC}"
        
        # Create backup
        cp "$file" "$file.backup"
        
        # Fix import statements
        sed -i 's|use crate::unified_enums::|use crate::canonical_modernization::|g' "$file"
        sed -i 's|use.*unified_enums::|use crate::canonical_modernization::|g' "$file"
        
        # Fix crate::unified_enums:: references
        sed -i 's|crate::unified_enums::|crate::canonical_modernization::|g' "$file"
        
        # Fix specific enum paths
        sed -i 's|unified_enums::UnifiedCapabilityType|canonical_modernization::UnifiedCapabilityType|g' "$file"
        sed -i 's|unified_enums::UnifiedServiceType|canonical_modernization::UnifiedServiceType|g' "$file"
        sed -i 's|unified_enums::UnifiedServiceState|canonical_modernization::UnifiedServiceState|g' "$file"
        sed -i 's|unified_enums::UnifiedHealthStatus|canonical_modernization::UnifiedHealthStatus|g' "$file"
        sed -i 's|unified_enums::UnifiedTierType|canonical_modernization::UnifiedTierType|g' "$file"
        
        # Fix nested module references
        sed -i 's|unified_enums::service_types::|canonical_modernization::|g' "$file"
        sed -i 's|unified_enums::health_types::|canonical_modernization::|g' "$file"
        
        changed=true
        ((FIXED_FILES++))
        
        # Remove backup
        rm "$file.backup"
        
        echo -e "${GREEN}✅ Fixed $file${NC}"
    fi
    
    ((TOTAL_FILES++))
}

# Process all Rust files
echo "🔍 Scanning Rust files..."
find . -name "*.rs" -type f | while read -r file; do
    fix_unified_enums_file "$file"
done

echo ""
echo "📊 Summary:"
echo "   Total files scanned: $TOTAL_FILES"
echo "   Files fixed: $FIXED_FILES"

# Check if any unified_enums references remain
REMAINING=$(find . -name "*.rs" -type f -exec grep -l "unified_enums::" {} \; 2>/dev/null | wc -l)

if [ "$REMAINING" -eq 0 ]; then
    echo -e "${GREEN}✅ All unified_enums references fixed!${NC}"
else
    echo -e "${RED}⚠️  $REMAINING files still contain unified_enums references${NC}"
    echo "Run: grep -r \"unified_enums::\" --include=\"*.rs\" ."
fi

echo -e "${GREEN}🚀 unified_enums migration complete!${NC}" 