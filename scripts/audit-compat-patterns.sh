#!/bin/bash
# audit-compat-patterns.sh
#
# Audits _compat, _shim, _helper, _legacy, and _old patterns
# Categorizes them for KEEP/REMOVE decisions
#
# Author: NestGate Team
# Date: November 8, 2025

set -e

echo "========================================="
echo "🔍 NestGate Compat Pattern Audit"
echo "========================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Output file
REPORT="compat_audit_report_$(date +%Y%m%d_%H%M%S).txt"

echo "📊 Analyzing compat patterns..."
echo ""

# Header
{
    echo "========================================"
    echo "NestGate Compat Pattern Audit Report"
    echo "Generated: $(date)"
    echo "========================================"
    echo ""
} > "$REPORT"

# 1. Total count
echo "1️⃣  TOTAL PATTERNS"
echo "----------------------------------------"
TOTAL=$(grep -r "_compat\|_shim\|_helper\|_legacy\|_old" code/crates/ \
    -i --exclude-dir=target 2>/dev/null | wc -l)
echo -e "   Total matches: ${YELLOW}${TOTAL}${NC}"

{
    echo "TOTAL MATCHES: $TOTAL"
    echo ""
} >> "$REPORT"

# 2. By pattern type
echo ""
echo "2️⃣  BY PATTERN TYPE"
echo "----------------------------------------"

for pattern in "_compat" "_shim" "_helper" "_legacy" "_old"; do
    COUNT=$(grep -ri "$pattern" code/crates/ --exclude-dir=target 2>/dev/null | wc -l)
    echo "   ${pattern}: $COUNT"
    echo "${pattern}: $COUNT" >> "$REPORT"
done

echo ""

# 3. By directory
echo "3️⃣  BY CRATE"
echo "----------------------------------------"
{
    echo ""
    echo "BY CRATE:"
    echo "--------"
} >> "$REPORT"

grep -r "_compat\|_shim\|_helper\|_legacy\|_old" code/crates/ \
    -i --exclude-dir=target 2>/dev/null | \
    cut -d: -f1 | \
    sed 's|code/crates/\([^/]*\).*|\1|' | \
    sort | uniq -c | sort -rn | head -20 | \
    while read -r count crate; do
        echo "   $crate: $count"
        echo "$crate: $count" >> "$REPORT"
    done

echo ""

# 4. Test files (likely KEEP)
echo "4️⃣  TEST FILES (likely KEEP)"
echo "----------------------------------------"
TEST_COUNT=$(grep -r "_helper\|_compat" code/crates/ \
    --include="*test*.rs" --include="*_tests.rs" \
    --exclude-dir=target 2>/dev/null | wc -l)
echo -e "   Test helpers: ${GREEN}${TEST_COUNT}${NC} (recommendation: KEEP)"

{
    echo ""
    echo "TEST FILES (likely KEEP): $TEST_COUNT"
    echo ""
} >> "$REPORT"

# 5. Migration patterns (likely REMOVE after migration)
echo ""
echo "5️⃣  MIGRATION PATTERNS (likely REMOVE)"
echo "----------------------------------------"
MIGRATION_COUNT=$(grep -ri "_compat\|_legacy\|_old\|migration.*helper" code/crates/ \
    --exclude-dir=target 2>/dev/null | wc -l)
echo -e "   Migration patterns: ${YELLOW}${MIGRATION_COUNT}${NC} (recommendation: REMOVE after unification)"

{
    echo "MIGRATION PATTERNS (likely REMOVE): $MIGRATION_COUNT"
    echo ""
} >> "$REPORT"

# 6. Top files with patterns
echo ""
echo "6️⃣  TOP FILES (need manual review)"
echo "----------------------------------------"
{
    echo "TOP FILES BY MATCH COUNT:"
    echo "------------------------"
} >> "$REPORT"

grep -r "_compat\|_shim\|_helper\|_legacy\|_old" code/crates/ \
    -i --exclude-dir=target 2>/dev/null | \
    cut -d: -f1 | sort | uniq -c | sort -rn | head -10 | \
    while read -r count file; do
        echo "   [$count] $file"
        echo "[$count] $file" >> "$REPORT"
    done

echo ""

# 7. Detailed listing
echo "7️⃣  DETAILED LISTING"
echo "----------------------------------------"
echo "   (Writing to $REPORT)"

{
    echo ""
    echo "========================================"
    echo "DETAILED MATCH LISTING"
    echo "========================================"
    echo ""
} >> "$REPORT"

# Categorized output
{
    echo "=== TEST FILES ==="
    grep -rn "_helper\|_compat" code/crates/ \
        --include="*test*.rs" --include="*_tests.rs" \
        --exclude-dir=target 2>/dev/null || echo "None"
    
    echo ""
    echo "=== MIGRATION PATTERNS ==="
    grep -rin "_legacy\|_old.*\|migration.*helper\|compat" code/crates/ \
        --exclude-dir=target 2>/dev/null | grep -v test || echo "None"
    
    echo ""
    echo "=== OTHER PATTERNS ==="
    grep -rin "_shim\|_helper" code/crates/ \
        --exclude-dir=target 2>/dev/null | \
        grep -v test | grep -v migration || echo "None"
} >> "$REPORT"

# Summary and recommendations
echo ""
echo "========================================="
echo "📊 SUMMARY & RECOMMENDATIONS"
echo "========================================="
echo ""

# Calculate estimates
KEEP_EST=$((TEST_COUNT))
REMOVE_EST=$((MIGRATION_COUNT - TEST_COUNT))
REVIEW_EST=$((TOTAL - TEST_COUNT - REMOVE_EST))

echo -e "Estimated breakdown:"
echo -e "  ${GREEN}KEEP${NC} (test infrastructure): ~${KEEP_EST}"
echo -e "  ${YELLOW}REMOVE${NC} (post-migration): ~${REMOVE_EST}"
echo -e "  ${BLUE}REVIEW${NC} (manual decision): ~${REVIEW_EST}"
echo ""

{
    echo ""
    echo "========================================"
    echo "RECOMMENDATIONS"
    echo "========================================"
    echo ""
    echo "ESTIMATED BREAKDOWN:"
    echo "  KEEP (test infrastructure): ~$KEEP_EST"
    echo "  REMOVE (post-migration): ~$REMOVE_EST"
    echo "  REVIEW (manual decision): ~$REVIEW_EST"
    echo ""
    echo "NEXT STEPS:"
    echo "1. Review detailed listing above"
    echo "2. For each pattern, decide: KEEP or REMOVE"
    echo "3. Document decision in code comments"
    echo "4. Remove unnecessary patterns"
    echo "5. Update tests if needed"
    echo ""
    echo "PRIORITY:"
    echo "- HIGH: Remove obvious migration helpers"
    echo "- MEDIUM: Review and clean up legacy compat"
    echo "- LOW: Document legitimate helpers"
} >> "$REPORT"

echo "Next steps:"
echo "1. Review full report: cat $REPORT"
echo "2. Manual review of top files"
echo "3. Categorize each pattern as KEEP/REMOVE"
echo "4. Remove unnecessary patterns"
echo "5. Document legitimate patterns"
echo ""

echo "========================================="
echo "✅ Audit complete!"
echo "========================================="
echo ""
echo "Report saved to: $REPORT"
echo ""
echo -e "${YELLOW}Note: This is a preliminary analysis.${NC}"
echo -e "${YELLOW}Manual review is required for final decisions.${NC}"
echo ""

