#!/bin/bash

# **NESTGATE UNIFICATION VALIDATION SCRIPT**
# 
# This script validates the progress of type, struct, trait, config, and error system
# unification across the NestGate codebase and identifies remaining technical debt.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Counters
TOTAL_ISSUES=0
FIXED_ISSUES=0

echo -e "${CYAN}🔍 **NESTGATE UNIFICATION VALIDATION REPORT**${NC}"
echo -e "${CYAN}============================================${NC}"
echo ""

# Function to report findings
report_finding() {
    local status=$1
    local category=$2
    local message=$3
    local count=${4:-0}
    
    if [[ $status == "GOOD" ]]; then
        echo -e "✅ ${GREEN}$category${NC}: $message"
        ((FIXED_ISSUES += count))
    elif [[ $status == "WARN" ]]; then
        echo -e "⚠️  ${YELLOW}$category${NC}: $message"
        ((TOTAL_ISSUES += count))
    else
        echo -e "❌ ${RED}$category${NC}: $message"
        ((TOTAL_ISSUES += count))
    fi
}

# Function to count occurrences
count_pattern() {
    local pattern=$1
    local include_pattern=${2:-"*.rs"}
    local exclude_pattern=${3:-"tests/*"}
    
    find code -name "$include_pattern" -not -path "$exclude_pattern" -exec grep -l "$pattern" {} \; 2>/dev/null | wc -l
}

# Function to count lines in files
count_lines() {
    local pattern=$1
    local include_pattern=${2:-"*.rs"}
    local exclude_pattern=${3:-"tests/*"}
    
    find code -name "$include_pattern" -not -path "$exclude_pattern" -exec grep -c "$pattern" {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}'
}

echo -e "${BLUE}## 1. ERROR SYSTEM UNIFICATION${NC}"
echo -e "${BLUE}===============================${NC}"

# Check for duplicate error types
duplicate_errors=$(count_pattern "enum.*Error.*{")
if [[ $duplicate_errors -eq 0 ]]; then
    report_finding "GOOD" "Error Types" "No duplicate error enums found"
else
    report_finding "WARN" "Error Types" "$duplicate_errors files still contain separate error enums" $duplicate_errors
fi

# Check for consolidated error builders
error_builders=$(count_pattern "ErrorBuilder")
if [[ $error_builders -gt 0 ]]; then
    report_finding "GOOD" "Error Builders" "$error_builders consolidated error builders found" $error_builders
else
    report_finding "WARN" "Error Builders" "No consolidated error builders found"
fi

# Check for deprecated error usage
deprecated_errors=$(count_lines "#\[deprecated.*error")
if [[ $deprecated_errors -gt 0 ]]; then
    report_finding "GOOD" "Deprecated Errors" "$deprecated_errors deprecated error items marked for migration" $deprecated_errors
fi

# Check Result type aliases
result_aliases=$(count_lines "type Result.*=")
if [[ $result_aliases -le 5 ]]; then
    report_finding "GOOD" "Result Types" "$result_aliases Result type aliases (consolidated)" $result_aliases
else
    report_finding "WARN" "Result Types" "$result_aliases Result type aliases (may need consolidation)" $result_aliases
fi

echo ""
echo -e "${BLUE}## 2. CONFIGURATION UNIFICATION${NC}"
echo -e "${BLUE}===============================${NC}"

# Check for Config struct definitions
config_structs=$(count_pattern "struct.*Config.*{")
echo "Config structs found: $config_structs"

# Check for consolidated config usage
consolidated_configs=$(count_pattern "ConsolidatedCanonicalConfig")
if [[ $consolidated_configs -gt 0 ]]; then
    report_finding "GOOD" "Consolidated Config" "$consolidated_configs files using ConsolidatedCanonicalConfig" $consolidated_configs
else
    report_finding "WARN" "Consolidated Config" "ConsolidatedCanonicalConfig not being used"
fi

# Check for deprecated config modules
deprecated_configs=$(count_lines "#\[deprecated.*config")
if [[ $deprecated_configs -gt 0 ]]; then
    report_finding "GOOD" "Deprecated Configs" "$deprecated_configs deprecated config items marked" $deprecated_configs
fi

echo ""
echo -e "${BLUE}## 3. TRAIT SYSTEM MODERNIZATION${NC}"
echo -e "${BLUE}===============================${NC}"

# Check for async_trait usage
async_trait_usage=$(count_pattern "async_trait")
if [[ $async_trait_usage -eq 0 ]]; then
    report_finding "GOOD" "Async Trait" "No async_trait usage found (fully migrated to native async)"
else
    report_finding "WARN" "Async Trait" "$async_trait_usage files still using async_trait" $async_trait_usage
fi

# Check for native async traits
native_async_traits=$(count_pattern "impl.*Future.*Output.*Send")
if [[ $native_async_traits -gt 0 ]]; then
    report_finding "GOOD" "Native Async" "$native_async_traits native async trait methods found" $native_async_traits
fi

# Check for unified canonical traits
canonical_traits=$(count_pattern "UnifiedCanonical.*Trait")
if [[ $canonical_traits -gt 0 ]]; then
    report_finding "GOOD" "Canonical Traits" "$canonical_traits unified canonical traits found" $canonical_traits
fi

echo ""
echo -e "${BLUE}## 4. FILE SIZE COMPLIANCE${NC}"
echo -e "${BLUE}=========================${NC}"

# Check file sizes (2000 line limit)
large_files=0
max_lines=0
largest_file=""

while IFS= read -r -d '' file; do
    lines=$(wc -l < "$file")
    if [[ $lines -gt 2000 ]]; then
        ((large_files++))
        if [[ $lines -gt $max_lines ]]; then
            max_lines=$lines
            largest_file=$file
        fi
    fi
done < <(find code -name "*.rs" -print0)

if [[ $large_files -eq 0 ]]; then
    report_finding "GOOD" "File Size" "All files under 2000 lines (excellent compliance)"
else
    report_finding "BAD" "File Size" "$large_files files exceed 2000 lines (largest: $largest_file with $max_lines lines)" $large_files
fi

echo ""
echo -e "${BLUE}## 5. TECHNICAL DEBT ANALYSIS${NC}"
echo -e "${BLUE}=============================${NC}"

# Check for TODO/FIXME markers
todo_count=$(count_lines "TODO\|FIXME\|HACK")
if [[ $todo_count -lt 20 ]]; then
    report_finding "GOOD" "Technical Debt" "$todo_count TODO/FIXME/HACK markers (manageable level)"
else
    report_finding "WARN" "Technical Debt" "$todo_count TODO/FIXME/HACK markers (needs attention)" $todo_count
fi

# Check for deprecated items
deprecated_items=$(count_lines "#\[deprecated")
if [[ $deprecated_items -gt 0 ]]; then
    report_finding "GOOD" "Deprecation" "$deprecated_items items properly marked as deprecated" $deprecated_items
fi

# Check for legacy/compatibility code
legacy_code=$(count_pattern "legacy\|compat\|shim")
if [[ $legacy_code -gt 0 ]]; then
    report_finding "WARN" "Legacy Code" "$legacy_code files contain legacy/compatibility code" $legacy_code
fi

echo ""
echo -e "${BLUE}## 6. PERFORMANCE OPTIMIZATIONS${NC}"
echo -e "${BLUE}===============================${NC}"

# Check for zero-cost abstractions
zero_cost=$(count_pattern "zero.cost\|ZeroCost")
if [[ $zero_cost -gt 0 ]]; then
    report_finding "GOOD" "Zero-Cost" "$zero_cost zero-cost optimization implementations found" $zero_cost
fi

# Check for SIMD usage
simd_usage=$(count_pattern "simd\|SIMD")
if [[ $simd_usage -gt 0 ]]; then
    report_finding "GOOD" "SIMD" "$simd_usage SIMD optimizations found" $simd_usage
fi

# Check for unsafe blocks (should be minimal and justified)
unsafe_blocks=$(count_lines "unsafe")
if [[ $unsafe_blocks -lt 50 ]]; then
    report_finding "GOOD" "Unsafe Code" "$unsafe_blocks unsafe blocks (reasonable for performance-critical code)"
else
    report_finding "WARN" "Unsafe Code" "$unsafe_blocks unsafe blocks (review for necessity)" $unsafe_blocks
fi

echo ""
echo -e "${BLUE}## 7. CONSOLIDATION ACHIEVEMENTS${NC}"
echo -e "${BLUE}===============================${NC}"

# Check for consolidated modules
consolidated_modules=$(count_pattern "CONSOLIDATION.*COMPLETE\|CONSOLIDATED.*:")
if [[ $consolidated_modules -gt 0 ]]; then
    report_finding "GOOD" "Consolidation" "$consolidated_modules consolidated modules identified" $consolidated_modules
fi

# Check for migration status comments
migration_status=$(count_pattern "MIGRATION STATUS.*✅")
if [[ $migration_status -gt 0 ]]; then
    report_finding "GOOD" "Migration Status" "$migration_status modules with completed migration status" $migration_status
fi

echo ""
echo -e "${PURPLE}## SUMMARY${NC}"
echo -e "${PURPLE}==========${NC}"

total_items=$((TOTAL_ISSUES + FIXED_ISSUES))
if [[ $total_items -gt 0 ]]; then
    completion_rate=$((FIXED_ISSUES * 100 / total_items))
else
    completion_rate=100
fi

echo -e "📊 ${CYAN}Unification Progress${NC}: ${GREEN}$completion_rate%${NC} ($FIXED_ISSUES/$total_items items completed)"
echo -e "⚠️  ${YELLOW}Remaining Issues${NC}: $TOTAL_ISSUES items need attention"
echo -e "✅ ${GREEN}Completed Items${NC}: $FIXED_ISSUES items successfully unified"

echo ""
if [[ $completion_rate -ge 80 ]]; then
    echo -e "🎉 ${GREEN}EXCELLENT PROGRESS!${NC} Your codebase is well-unified and modernized."
    echo -e "   Focus on cleaning up the remaining $TOTAL_ISSUES items for full completion."
elif [[ $completion_rate -ge 60 ]]; then
    echo -e "👍 ${YELLOW}GOOD PROGRESS!${NC} You're on the right track with unification."
    echo -e "   Continue working on the remaining $TOTAL_ISSUES items."
else
    echo -e "⚠️  ${RED}MORE WORK NEEDED${NC} Continue with systematic unification efforts."
fi

echo ""
echo -e "${CYAN}## NEXT STEPS${NC}"
echo -e "${CYAN}=============${NC}"

if [[ $async_trait_usage -gt 0 ]]; then
    echo -e "1. 🔄 Migrate remaining $async_trait_usage async_trait usages to native async"
fi

if [[ $legacy_code -gt 0 ]]; then
    echo -e "2. 🧹 Clean up $legacy_code legacy/compatibility code files"
fi

if [[ $todo_count -gt 20 ]]; then
    echo -e "3. 📝 Address high TODO/FIXME count ($todo_count items)"
fi

if [[ $large_files -gt 0 ]]; then
    echo -e "4. ✂️  Split $large_files files exceeding 2000 lines"
fi

echo -e "5. 🚀 Continue with systematic modernization and optimization"

echo ""
echo -e "${CYAN}Report completed: $(date)${NC}" 