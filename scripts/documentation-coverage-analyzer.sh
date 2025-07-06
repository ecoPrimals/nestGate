#!/bin/bash

echo "🎯 NestGate Documentation Coverage Analyzer"
echo "==========================================="

WORKSPACE_ROOT="${PWD}"
TOTAL_ITEMS=0
DOCUMENTED_ITEMS=0
MISSING_DOCS=()

# Function to analyze Rust file for documentation coverage
analyze_rust_file() {
    local file="$1"
    local items=0
    local documented=0
    
    echo "📄 Analyzing: $file"
    
    # Count public structs, enums, functions, methods, modules, traits
    local pub_items=$(grep -E "^[[:space:]]*pub[[:space:]]+(struct|enum|fn|mod|trait|type|const|static)" "$file" | wc -l)
    local impl_methods=$(grep -E "^[[:space:]]*pub[[:space:]]+fn[[:space:]]+" "$file" | wc -l)
    
    items=$((pub_items + impl_methods))
    
    # Count documented items (those with /// or /** above them)
    local doc_comments=$(grep -B1 -E "^[[:space:]]*pub[[:space:]]+(struct|enum|fn|mod|trait|type|const|static)" "$file" | grep -E "^[[:space:]]*///" | wc -l)
    local doc_methods=$(grep -B1 -E "^[[:space:]]*pub[[:space:]]+fn[[:space:]]+" "$file" | grep -E "^[[:space:]]*///" | wc -l)
    
    documented=$((doc_comments + doc_methods))
    
    if [ $items -gt 0 ]; then
        local coverage=$(echo "scale=1; $documented * 100 / $items" | bc)
        echo "  📊 Items: $items, Documented: $documented, Coverage: ${coverage}%"
        
        if [ $documented -lt $items ]; then
            MISSING_DOCS+=("$file: Missing $(($items - $documented)) documentation comments")
        fi
    fi
    
    TOTAL_ITEMS=$((TOTAL_ITEMS + items))
    DOCUMENTED_ITEMS=$((DOCUMENTED_ITEMS + documented))
}

# Find all Rust source files
echo "🔍 Scanning for Rust source files..."
find code/crates -name "*.rs" -type f | while read -r file; do
    if [[ ! "$file" == *"/target/"* ]] && [[ ! "$file" == *"tests.rs"* ]]; then
        analyze_rust_file "$file"
    fi
done

# Also scan main workspace files
find src -name "*.rs" -type f 2>/dev/null | while read -r file; do
    analyze_rust_file "$file"
done

echo ""
echo "📊 Overall Documentation Coverage Summary"
echo "========================================"
if [ $TOTAL_ITEMS -gt 0 ]; then
    COVERAGE=$(echo "scale=1; $DOCUMENTED_ITEMS * 100 / $TOTAL_ITEMS" | bc)
    echo "📈 Total Items: $TOTAL_ITEMS"
    echo "📝 Documented Items: $DOCUMENTED_ITEMS" 
    echo "📊 Coverage: ${COVERAGE}%"
    echo "🎯 Target: 100%"
    echo "📉 Gap: $(($TOTAL_ITEMS - $DOCUMENTED_ITEMS)) items"
else
    echo "⚠️  No public items found"
fi

echo ""
echo "📋 Missing Documentation Report"
echo "=============================="
if [ ${#MISSING_DOCS[@]} -gt 0 ]; then
    for missing in "${MISSING_DOCS[@]}"; do
        echo "❌ $missing"
    done
else
    echo "✅ All public items documented!"
fi

# Suggest specific improvements
echo ""
echo "💡 Documentation Improvement Suggestions"
echo "======================================="
echo "1. Add /// comments above all pub struct, enum, fn, trait definitions"
echo "2. Add module-level documentation with //! comments"
echo "3. Include examples in documentation where helpful"
echo "4. Document error conditions and panics"
echo "5. Add links between related types using [Type] syntax" 