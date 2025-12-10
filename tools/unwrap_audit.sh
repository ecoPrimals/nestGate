#!/usr/bin/env bash
# Unwrap Audit Tool - December 10, 2025
# Comprehensive audit of .unwrap() and .expect() usage
# Categorizes by: Production vs Test, File, Priority

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CODE_DIR="$PROJECT_ROOT/code"

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo "═══════════════════════════════════════════════════════════════"
echo "  UNWRAP/EXPECT AUDIT - Production Code Analysis"
echo "  Date: $(date '+%Y-%m-%d %H:%M:%S')"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Function to count unwraps in a file
count_unwraps() {
    local file="$1"
    grep -n "\.unwrap()" "$file" 2>/dev/null | wc -l || echo "0"
}

# Function to count expects in a file
count_expects() {
    local file="$1"
    grep -n "\.expect(" "$file" 2>/dev/null | wc -l || echo "0"
}

# Function to check if file is a test file
is_test_file() {
    local file="$1"
    [[ "$file" == *"_test.rs" ]] || \
    [[ "$file" == *"_tests.rs" ]] || \
    [[ "$file" == *"/tests/"* ]] || \
    [[ "$file" == *"/benches/"* ]]
}

# Arrays to store results
declare -a CRITICAL_FILES=()
declare -a HIGH_PRIORITY_FILES=()
declare -a MEDIUM_PRIORITY_FILES=()
declare -a TEST_FILES=()

# Counters
TOTAL_PRODUCTION_UNWRAPS=0
TOTAL_PRODUCTION_EXPECTS=0
TOTAL_TEST_UNWRAPS=0
TOTAL_TEST_EXPECTS=0

echo "🔍 Scanning for .unwrap() and .expect() calls..."
echo ""

# Find all Rust files
while IFS= read -r -d '' file; do
    UNWRAP_COUNT=$(count_unwraps "$file")
    EXPECT_COUNT=$(count_expects "$file")
    TOTAL_COUNT=$((UNWRAP_COUNT + EXPECT_COUNT))
    
    if [ "$TOTAL_COUNT" -gt 0 ]; then
        RELATIVE_PATH="${file#$PROJECT_ROOT/}"
        
        if is_test_file "$file"; then
            # Test file
            TEST_FILES+=("$TOTAL_COUNT|$UNWRAP_COUNT|$EXPECT_COUNT|$RELATIVE_PATH")
            TOTAL_TEST_UNWRAPS=$((TOTAL_TEST_UNWRAPS + UNWRAP_COUNT))
            TOTAL_TEST_EXPECTS=$((TOTAL_TEST_EXPECTS + EXPECT_COUNT))
        else
            # Production file
            TOTAL_PRODUCTION_UNWRAPS=$((TOTAL_PRODUCTION_UNWRAPS + UNWRAP_COUNT))
            TOTAL_PRODUCTION_EXPECTS=$((TOTAL_PRODUCTION_EXPECTS + EXPECT_COUNT))
            
            # Categorize by priority
            if [[ "$file" == *"/network/client.rs" ]] || \
               [[ "$file" == *"/api/handlers/"* ]] || \
               [[ "$file" == *"/primal_discovery"* ]]; then
                CRITICAL_FILES+=("$TOTAL_COUNT|$UNWRAP_COUNT|$EXPECT_COUNT|$RELATIVE_PATH")
            elif [ "$TOTAL_COUNT" -ge 10 ]; then
                HIGH_PRIORITY_FILES+=("$TOTAL_COUNT|$UNWRAP_COUNT|$EXPECT_COUNT|$RELATIVE_PATH")
            else
                MEDIUM_PRIORITY_FILES+=("$TOTAL_COUNT|$UNWRAP_COUNT|$EXPECT_COUNT|$RELATIVE_PATH")
            fi
        fi
    fi
done < <(find "$CODE_DIR/crates" -name "*.rs" -type f -print0 2>/dev/null)

# Summary Statistics
echo "═══════════════════════════════════════════════════════════════"
echo "  📊 SUMMARY STATISTICS"
echo "═══════════════════════════════════════════════════════════════"
echo ""
printf "${RED}Production Code:${NC}\n"
printf "  .unwrap():  %4d calls\n" "$TOTAL_PRODUCTION_UNWRAPS"
printf "  .expect():  %4d calls\n" "$TOTAL_PRODUCTION_EXPECTS"
printf "  ${RED}TOTAL:      %4d calls${NC}\n" "$((TOTAL_PRODUCTION_UNWRAPS + TOTAL_PRODUCTION_EXPECTS))"
echo ""
printf "${GREEN}Test Code:${NC}\n"
printf "  .unwrap():  %4d calls\n" "$TOTAL_TEST_UNWRAPS"
printf "  .expect():  %4d calls\n" "$TOTAL_TEST_EXPECTS"
printf "  TOTAL:      %4d calls\n" "$((TOTAL_TEST_UNWRAPS + TOTAL_TEST_EXPECTS))"
echo ""
printf "${YELLOW}Grand Total:  %4d calls${NC}\n" "$((TOTAL_PRODUCTION_UNWRAPS + TOTAL_PRODUCTION_EXPECTS + TOTAL_TEST_UNWRAPS + TOTAL_TEST_EXPECTS))"
echo ""

# Critical Files (Hot Paths)
if [ ${#CRITICAL_FILES[@]} -gt 0 ]; then
    echo "═══════════════════════════════════════════════════════════════"
    echo "  🚨 CRITICAL PRIORITY (Hot Paths)"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    printf "%-60s %8s %8s %8s\n" "File" "Total" ".unwrap()" ".expect()"
    echo "───────────────────────────────────────────────────────────────"
    
    for entry in "${CRITICAL_FILES[@]}"; do
        IFS='|' read -r total unwraps expects path <<< "$entry"
        printf "${RED}%-60s %8s %8s %8s${NC}\n" "$path" "$total" "$unwraps" "$expects"
    done
    echo ""
fi

# High Priority Files
if [ ${#HIGH_PRIORITY_FILES[@]} -gt 0 ]; then
    echo "═══════════════════════════════════════════════════════════════"
    echo "  ⚠️  HIGH PRIORITY (10+ calls)"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    printf "%-60s %8s %8s %8s\n" "File" "Total" ".unwrap()" ".expect()"
    echo "───────────────────────────────────────────────────────────────"
    
    # Sort by total count (descending)
    for entry in "${HIGH_PRIORITY_FILES[@]}"; do
        echo "$entry"
    done | sort -t'|' -k1 -rn | while IFS='|' read -r total unwraps expects path; do
        printf "${YELLOW}%-60s %8s %8s %8s${NC}\n" "$path" "$total" "$unwraps" "$expects"
    done
    echo ""
fi

# Migration Recommendations
echo "═══════════════════════════════════════════════════════════════"
echo "  💡 MIGRATION RECOMMENDATIONS"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Phase 1: Critical Paths (Week 2)"
echo "  • network/client.rs"
echo "  • api/handlers/*"
echo "  • primal_discovery/*"
echo "  Impact: HIGH - These are hot paths"
echo ""
echo "Phase 2: High Priority (Week 3-4)"
echo "  • Files with 10+ unwraps/expects"
echo "  Impact: MEDIUM - Reduce panic risk"
echo ""
echo "Phase 3: Medium Priority (Week 5-6)"
echo "  • Remaining production files"
echo "  Impact: LOW - Complete migration"
echo ""

# Migration Pattern Example
echo "═══════════════════════════════════════════════════════════════"
echo "  🔄 MIGRATION PATTERN"
echo "═══════════════════════════════════════════════════════════════"
echo ""
cat << 'EOF'
// ❌ OLD: Panics in production
let value = some_operation().unwrap();

// ✅ NEW: Idiomatic error handling
let value = some_operation()
    .context("Failed to perform operation")?;

// Or with custom error
let value = some_operation()
    .map_err(|e| NestGateError::operation_failed(&e.to_string()))?;
EOF
echo ""

# Save detailed report to file
REPORT_FILE="$PROJECT_ROOT/unwrap_audit_report_$(date +%Y%m%d_%H%M%S).txt"
{
    echo "UNWRAP/EXPECT AUDIT REPORT"
    echo "Generated: $(date)"
    echo ""
    echo "PRODUCTION CODE:"
    echo "  .unwrap(): $TOTAL_PRODUCTION_UNWRAPS"
    echo "  .expect(): $TOTAL_PRODUCTION_EXPECTS"
    echo "  TOTAL: $((TOTAL_PRODUCTION_UNWRAPS + TOTAL_PRODUCTION_EXPECTS))"
    echo ""
    echo "TEST CODE:"
    echo "  .unwrap(): $TOTAL_TEST_UNWRAPS"
    echo "  .expect(): $TOTAL_TEST_EXPECTS"
    echo "  TOTAL: $((TOTAL_TEST_UNWRAPS + TOTAL_TEST_EXPECTS))"
    echo ""
    echo "CRITICAL FILES:"
    for entry in "${CRITICAL_FILES[@]}"; do
        IFS='|' read -r total unwraps expects path <<< "$entry"
        echo "  $path: $total total ($unwraps unwrap, $expects expect)"
    done
    echo ""
    echo "HIGH PRIORITY FILES:"
    for entry in "${HIGH_PRIORITY_FILES[@]}"; do
        IFS='|' read -r total unwraps expects path <<< "$entry"
        echo "  $path: $total total ($unwraps unwrap, $expects expect)"
    done
} > "$REPORT_FILE"

echo "═══════════════════════════════════════════════════════════════"
echo "  📄 REPORT SAVED"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Detailed report saved to:"
echo "  ${REPORT_FILE#$PROJECT_ROOT/}"
echo ""

# Exit with error if too many production unwraps
if [ "$TOTAL_PRODUCTION_UNWRAPS" -gt 1000 ]; then
    echo "${RED}⚠️  WARNING: More than 1,000 production .unwrap() calls found!${NC}"
    echo "Consider prioritizing migration work."
    exit 1
fi

echo "${GREEN}✅ Audit complete!${NC}"
echo ""

