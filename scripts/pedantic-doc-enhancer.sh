#!/bin/bash

# PEDANTIC DOCUMENTATION ENHANCER
# Automatically adds missing # Errors sections to Result-returning functions

set -euo pipefail

echo "🎯 PEDANTIC DOCUMENTATION ENHANCEMENT"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counter for improvements
IMPROVEMENTS=0

# Function to add # Errors documentation
add_errors_doc() {
    local file="$1"
    local line_num="$2"
    local function_signature="$3"
    
    echo -e "${BLUE}📝 Adding # Errors documentation to: ${function_signature}${NC}"
    
    # Create a temporary file with the enhanced documentation
    local temp_file=$(mktemp)
    
    # Add the # Errors section before the function
    head -n $((line_num - 1)) "$file" > "$temp_file"
    echo "    /// # Errors" >> "$temp_file"
    echo "    /// " >> "$temp_file"
    echo "    /// Returns an error if the operation fails or encounters invalid input." >> "$temp_file"
    tail -n +$line_num "$file" >> "$temp_file"
    
    # Replace the original file
    mv "$temp_file" "$file"
    
    ((IMPROVEMENTS++))
}

# Find all Rust files in the codebase
find code/crates -name "*.rs" -type f | while read -r file; do
    echo -e "${YELLOW}🔍 Analyzing: $file${NC}"
    
    # Look for functions returning Result that lack # Errors documentation
    grep -n "-> Result<" "$file" | while IFS=: read -r line_num line_content; do
        # Check if the previous lines contain "# Errors"
        if ! head -n $((line_num - 1)) "$file" | tail -n 10 | grep -q "# Errors"; then
            echo -e "${RED}⚠️  Missing # Errors documentation at line $line_num${NC}"
            # Extract function name from the line
            function_name=$(echo "$line_content" | sed -n 's/.*fn \([^(]*\).*/\1/p')
            if [[ -n "$function_name" ]]; then
                add_errors_doc "$file" "$line_num" "$function_name"
            fi
        fi
    done
done

echo ""
echo -e "${GREEN}✅ PEDANTIC DOCUMENTATION ENHANCEMENT COMPLETE${NC}"
echo -e "${GREEN}📊 Total improvements made: $IMPROVEMENTS${NC}"
echo ""
echo "🎯 Next steps:"
echo "  1. Review the added documentation"
echo "  2. Customize error descriptions for specific functions"
echo "  3. Run 'cargo clippy' to verify improvements"
echo "" 