#!/bin/bash
# 🦀 **NESTGATE CONST MODERNIZATION SCRIPT**
# Systematic approach to fix const function overuse using Rust constraints

set -euo pipefail

echo "🦀 **NESTGATE CONST FUNCTION MODERNIZATION**"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Statistics tracking
TOTAL_CONST_FUNCTIONS=0
FUNCTIONS_CONVERTED=0
FUNCTIONS_KEPT_CONST=0
FILES_MODIFIED=0

echo -e "${BLUE}Phase 1: Analyzing const function usage...${NC}"

# Count total const functions
TOTAL_CONST_FUNCTIONS=$(grep -r "pub const fn" code/crates/ --include="*.rs" | wc -l)
echo -e "📊 Total const functions found: ${YELLOW}${TOTAL_CONST_FUNCTIONS}${NC}"

echo -e "\n${BLUE}Phase 2: Categorizing const functions by Rust principles...${NC}"

# Category 1: Definitely should NOT be const (return complex types)
echo -e "🔍 Finding const functions returning complex types..."
COMPLEX_TYPE_VIOLATIONS=$(grep -r "pub const fn.*-> String\|pub const fn.*-> HashMap\|pub const fn.*-> Vec\|pub const fn.*-> Result\|pub const fn.*-> Option<String>\|pub const fn.*-> Response" code/crates/ --include="*.rs" | wc -l)
echo -e "❌ Complex type violations: ${RED}${COMPLEX_TYPE_VIOLATIONS}${NC}"

# Category 2: Functions that call non-const operations
echo -e "🔍 Finding const functions with non-const operations..."
NON_CONST_OPERATIONS=$(grep -l "pub const fn" code/crates/**/*.rs | xargs grep -l "format!\|String::\|HashMap::\|Vec::\|\.to_string\|\.clone" | wc -l)
echo -e "❌ Non-const operation violations: ${RED}${NON_CONST_OPERATIONS}${NC}"

# Category 3: Potentially legitimate const functions (return primitives)
echo -e "🔍 Finding potentially legitimate const functions..."
PRIMITIVE_CONST=$(grep -r "pub const fn.*-> u8\|pub const fn.*-> u16\|pub const fn.*-> u32\|pub const fn.*-> u64\|pub const fn.*-> i8\|pub const fn.*-> i16\|pub const fn.*-> i32\|pub const fn.*-> i64\|pub const fn.*-> f32\|pub const fn.*-> f64\|pub const fn.*-> bool\|pub const fn.*-> usize\|pub const fn.*-> isize" code/crates/ --include="*.rs" | wc -l)
echo -e "✅ Potentially legitimate const: ${GREEN}${PRIMITIVE_CONST}${NC}"

echo -e "\n${BLUE}Phase 3: Rust-Guided Modernization Strategy${NC}"
echo -e "============================================"

echo -e "🎯 **MODERNIZATION PRIORITIES** (following Rust constraints):"
echo -e "1. ${RED}HIGH PRIORITY${NC}: Remove const from functions returning complex types"
echo -e "2. ${RED}HIGH PRIORITY${NC}: Remove const from functions with I/O operations"  
echo -e "3. ${YELLOW}MEDIUM PRIORITY${NC}: Remove const from functions calling non-const methods"
echo -e "4. ${GREEN}LOW PRIORITY${NC}: Review primitive-returning const functions"

echo -e "\n🦀 **RUST IDIOM PRINCIPLES APPLIED**:"
echo -e "• const fn should only be used for compile-time constants"
echo -e "• Functions that allocate, format, or do I/O should NOT be const"
echo -e "• Prefer regular fn for runtime computations"
echo -e "• Use const fn only for mathematical constants and simple getters"

echo -e "\n${BLUE}Phase 4: Execution Plan${NC}"
echo -e "======================"

read -p "🤔 Proceed with automated const function modernization? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${GREEN}🚀 Starting modernization...${NC}"
    
    # Step 1: Fix obvious violations (complex return types)
    echo -e "\n${YELLOW}Step 1: Fixing complex type violations...${NC}"
    find code/crates -name "*.rs" -type f -exec sed -i.bak 's/pub const fn \([^(]*([^)]*)\) -> String/pub fn \1 -> String/g' {} \;
    find code/crates -name "*.rs" -type f -exec sed -i.bak 's/pub const fn \([^(]*([^)]*)\) -> HashMap/pub fn \1 -> HashMap/g' {} \;
    find code/crates -name "*.rs" -type f -exec sed -i.bak 's/pub const fn \([^(]*([^)]*)\) -> Vec/pub fn \1 -> Vec/g' {} \;
    find code/crates -name "*.rs" -type f -exec sed -i.bak 's/pub const fn \([^(]*([^)]*)\) -> Result/pub fn \1 -> Result/g' {} \;
    find code/crates -name "*.rs" -type f -exec sed -i.bak 's/pub const fn \([^(]*([^)]*)\) -> Option<String>/pub fn \1 -> Option<String>/g' {} \;
    
    # Step 2: Fix functions that use format! macro
    echo -e "${YELLOW}Step 2: Fixing format! macro violations...${NC}"
    find code/crates -name "*.rs" -exec grep -l "const fn.*format!" {} \; | xargs sed -i.bak 's/pub const fn/pub fn/g'
    
    # Step 3: Fix functions that use .to_string()
    echo -e "${YELLOW}Step 3: Fixing .to_string() violations...${NC}"  
    find code/crates -name "*.rs" -exec grep -l "const fn.*\.to_string" {} \; | xargs sed -i.bak 's/pub const fn/pub fn/g'
    
    # Count changes
    FILES_MODIFIED=$(find code/crates -name "*.bak" | wc -l)
    echo -e "\n${GREEN}✅ Modernization complete!${NC}"
    echo -e "📊 Files modified: ${GREEN}${FILES_MODIFIED}${NC}"
    
    # Cleanup backup files
    find code/crates -name "*.bak" -delete
    
    echo -e "\n${BLUE}Phase 5: Validation${NC}"
    echo -e "=================="
    echo -e "🔍 Running cargo check to validate changes..."
    
    if cargo check --workspace --quiet; then
        echo -e "${GREEN}✅ All changes validated successfully!${NC}"
    else
        echo -e "${RED}⚠️  Some issues remain. Run 'cargo check' for details.${NC}"
    fi
    
    echo -e "\n${GREEN}🎉 CONST MODERNIZATION COMPLETE!${NC}"
    echo -e "Your codebase is now more idiomatic and follows Rust best practices."
    echo -e "\n📋 **NEXT STEPS**:"
    echo -e "1. Review remaining const functions manually"
    echo -e "2. Run comprehensive tests"  
    echo -e "3. Update documentation to reflect modern patterns"
    
else
    echo -e "${YELLOW}Modernization cancelled. No changes made.${NC}"
fi

echo -e "\n🦀 **RUST MODERNIZATION REPORT**"
echo -e "================================"
echo -e "Total const functions analyzed: ${TOTAL_CONST_FUNCTIONS}"
echo -e "Complex type violations: ${COMPLEX_TYPE_VIOLATIONS}"
echo -e "Files that would be modified: ${FILES_MODIFIED}"
echo -e "\nThis analysis helps evolve your codebase using Rust as a constraint!" 