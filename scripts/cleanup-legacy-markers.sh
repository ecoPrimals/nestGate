#!/bin/bash

# **LEGACY CLEANUP SCRIPT**
#
# This script systematically removes completed migration comments and legacy markers
# from the NestGate codebase as part of the final unification phase.

set -e

echo "🧹 Starting Legacy Marker Cleanup..."

# Function to clean up specific patterns
cleanup_pattern() {
    local pattern="$1"
    local description="$2"
    
    echo "  🔍 Cleaning up: $description"
    
    # Find files with the pattern and process them
    find code/ -name "*.rs" -type f -exec grep -l "$pattern" {} \; | while read -r file; do
        echo "    📝 Processing: $file"
        # Use sed to remove lines containing the pattern
        sed -i "/$pattern/d" "$file"
    done
}

# Function to clean up entire comment blocks
cleanup_comment_blocks() {
    echo "  🔍 Cleaning up completed migration comment blocks"
    
    find code/ -name "*.rs" -type f -exec grep -l "CANONICAL MODERNIZATION COMPLETE" {} \; | while read -r file; do
        echo "    📝 Processing migration comments in: $file"
        
        # Remove multi-line comment blocks about completed migrations
        sed -i '/\/\/ \*\*CANONICAL MODERNIZATION COMPLETE\*\*/,/\/\/ \*\*PROVIDES\*\*:/d' "$file"
        sed -i '/\/\/ \*\*CONSOLIDATES AND ELIMINATES\*\*:/,/\/\/ \*\*PROVIDES\*\*:/d' "$file"
        sed -i '/\/\/ \*\*MIGRATED\*\*:/d' "$file"
    done
}

# Function to clean up outdated TODO comments
cleanup_outdated_todos() {
    echo "  🔍 Cleaning up outdated TODO comments"
    
    # Remove specific completed TODO patterns
    cleanup_pattern "// TODO.*migration.*complete" "Completed migration TODOs"
    cleanup_pattern "// TODO.*consolidat.*complete" "Completed consolidation TODOs"
    cleanup_pattern "// FIXME.*async_trait.*removed" "Fixed async_trait issues"
}

# Function to clean up redundant documentation
cleanup_redundant_docs() {
    echo "  🔍 Cleaning up redundant documentation comments"
    
    # Remove excessive "ELIMINATES AND REPLACES" lists
    find code/ -name "*.rs" -type f -exec grep -l "ELIMINATES AND REPLACES" {} \; | while read -r file; do
        echo "    📝 Cleaning redundant docs in: $file"
        
        # Remove the verbose elimination lists, keep just the essential info
        sed -i '/\/\/ \*\*ELIMINATES AND REPLACES\*\*:/,/\/\/ \*\*PROVIDES\*\*:/c\
// **UNIFIED SYSTEM** - Replaces fragmented implementations with canonical patterns' "$file"
    done
}

# Function to update import comments
update_import_comments() {
    echo "  🔍 Updating import comments"
    
    find code/ -name "*.rs" -type f -exec grep -l "// REMOVED:" {} \; | while read -r file; do
        echo "    📝 Cleaning import comments in: $file"
        
        # Remove "REMOVED:" comments for imports
        sed -i '/\/\/ REMOVED:/d' "$file"
        sed -i '/\/\/ Removed:/d' "$file"
    done
}

# Function to consolidate success markers
consolidate_success_markers() {
    echo "  🔍 Consolidating success markers"
    
    find code/ -name "*.rs" -type f -exec grep -l "✅" {} \; | while read -r file; do
        echo "    📝 Consolidating markers in: $file"
        
        # Replace multiple success markers with a single consolidated comment
        sed -i 's/\/\/ - .* ✅/\/\/ ✅ Migrated/g' "$file"
        
        # Remove excessive checkmark lists
        sed -i '/\/\/ ✅.*✅.*✅/d' "$file"
    done
}

# Main cleanup execution
echo "📋 Phase 1: Cleaning up completed migration comments..."
cleanup_comment_blocks

echo "📋 Phase 2: Cleaning up outdated TODO/FIXME comments..."
cleanup_outdated_todos

echo "📋 Phase 3: Cleaning up redundant documentation..."
cleanup_redundant_docs

echo "📋 Phase 4: Updating import comments..."
update_import_comments

echo "📋 Phase 5: Consolidating success markers..."
consolidate_success_markers

# Additional specific cleanups
echo "📋 Phase 6: Specific pattern cleanups..."

# Remove specific verbose patterns
cleanup_pattern "CANONICAL MODERNIZATION COMPLETE.*eliminated" "Verbose completion messages"
cleanup_pattern "Migration complete.*use canonical" "Verbose migration messages"
cleanup_pattern "All functionality migrated to" "Migration status messages"

# Clean up excessive separator comments
echo "  🔍 Cleaning up excessive separator comments"
find code/ -name "*.rs" -type f -exec sed -i '/^\/\/ =\{20,\}/c\
// ==================== SECTION ====================' {} \;

echo "📋 Phase 7: Final validation..."

# Count remaining legacy markers
echo "📊 Remaining legacy markers:"
echo "  TODO: $(grep -r "TODO" code/ --include="*.rs" | wc -l)"
echo "  FIXME: $(grep -r "FIXME" code/ --include="*.rs" | wc -l)"
echo "  DEPRECATED: $(grep -r "DEPRECATED" code/ --include="*.rs" | wc -l)"
echo "  LEGACY: $(grep -r "LEGACY" code/ --include="*.rs" | wc -l)"

echo "✅ Legacy marker cleanup completed!"
echo ""
echo "📈 Summary:"
echo "  - Removed completed migration comment blocks"
echo "  - Cleaned up outdated TODO/FIXME comments"  
echo "  - Consolidated redundant documentation"
echo "  - Updated import comments"
echo "  - Consolidated success markers"
echo "  - Applied specific pattern cleanups"
echo ""
echo "🎯 Next steps:"
echo "  1. Review remaining legacy markers manually"
echo "  2. Update any remaining TODO items"
echo "  3. Run cargo clippy to check for new warnings"
echo "  4. Run tests to ensure functionality is preserved" 