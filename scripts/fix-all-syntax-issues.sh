#!/bin/bash
# 🔧 COMPREHENSIVE SYNTAX FIXES SCRIPT
# Fixes all remaining syntax and doc comment issues

set -euo pipefail

echo "🔧 **COMPREHENSIVE SYNTAX FIXES**"
echo "================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to fix doc comments in a file
fix_doc_comments() {
    local file="$1"
    if [ -f "$file" ]; then
        echo "📝 Fixing doc comments in $file..."
        # Create temp file with proper doc comment structure
        temp_file=$(mktemp)
        
        # Extract inner doc comments and put them at the top
        grep "^//!" "$file" > "$temp_file" 2>/dev/null || true
        echo "" >> "$temp_file"
        
        # Add non-doc-comment content
        grep -v "^//!" "$file" >> "$temp_file" 2>/dev/null || true
        
        mv "$temp_file" "$file"
    fi
}

# Fix specific files with doc comment issues
fix_doc_comments "code/crates/nestgate-core/src/zero_cost/system.rs"
fix_doc_comments "code/crates/nestgate-core/src/zero_cost/types.rs" 
fix_doc_comments "code/crates/nestgate-core/src/simd/types.rs"

# Fix the service_discovery mod issue
if [ -f "code/crates/nestgate-core/src/service_discovery/mod.rs" ]; then
    echo "📝 Fixing service_discovery/mod.rs line 6..."
    sed -i '6s/^\/\/!/\/\/\//' code/crates/nestgate-core/src/service_discovery/mod.rs
fi

# Fix the unified_config_consolidation syntax error
if [ -f "code/crates/nestgate-core/src/unified_config_consolidation.rs" ]; then
    echo "📝 Fixing unified_config_consolidation.rs syntax..."
    # Check if line 2 has a syntax error and fix it
    sed -i '2s/^{use super::/use super::/' code/crates/nestgate-core/src/unified_config_consolidation.rs
    sed -i '2s/;}/;/' code/crates/nestgate-core/src/unified_config_consolidation.rs
fi

echo "✅ **ALL SYNTAX FIXES APPLIED**"
echo "==============================="

# Test compilation
echo "🧪 Testing compilation..."
if cargo check --package nestgate-core --message-format short 2>&1 | head -5; then
    echo "✅ Compilation test completed"
else
    echo "⚠️  Some issues may remain"
fi 