#!/bin/bash
# NestGate Unification Status Check
# Quick status verification for unification progress

set -euo pipefail

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   🎯 NESTGATE UNIFICATION STATUS CHECK"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "📊 CORE METRICS"
echo "───────────────────────────────────────────────"

# 1. Build Status
echo -n "Build Status:        "
if cargo check --workspace 2>&1 | grep -q "error"; then
    echo -e "${RED}❌ ERRORS FOUND${NC}"
else
    echo -e "${GREEN}✅ GREEN${NC}"
fi

# 2. Test Status
echo -n "Test Status:         "
TEST_OUTPUT=$(cargo test --workspace --lib 2>&1 | grep "test result" | tail -1 || echo "error")
if echo "$TEST_OUTPUT" | grep -q "FAILED"; then
    echo -e "${RED}❌ FAILURES${NC}"
elif echo "$TEST_OUTPUT" | grep -q "ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep -o "[0-9]* passed" | grep -o "[0-9]*" || echo "0")
    echo -e "${GREEN}✅ ${PASSED} PASSING${NC}"
else
    echo -e "${YELLOW}⚠️  UNKNOWN${NC}"
fi

# 3. File Size Compliance
echo -n "File Size Compliance: "
OVERSIZED=$(find code/crates -name "*.rs" -exec wc -l {} + 2>/dev/null | awk '$1 > 2000 {count++} END {print count+0}')
if [ "$OVERSIZED" -eq 0 ]; then
    MAX_SIZE=$(find code/crates -name "*.rs" -exec wc -l {} + 2>/dev/null | sort -rn | head -1 | awk '{print $1}' || echo "0")
    echo -e "${GREEN}✅ ALL < 2000 lines (max: ${MAX_SIZE})${NC}"
else
    echo -e "${RED}❌ ${OVERSIZED} files > 2000 lines${NC}"
fi

# 4. Deprecated Code
echo -n "Deprecated Code:     "
if [ -f "code/crates/nestgate-core/src/unified_config_consolidation.rs" ]; then
    DEPRECATED_LINES=$(wc -l code/crates/nestgate-core/src/unified_config_consolidation.rs \
                           code/crates/nestgate-core/src/traits_root/mod.rs \
                           code/crates/nestgate-core/src/error/idiomatic/mod.rs \
                           2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    echo -e "${YELLOW}⏳ ${DEPRECATED_LINES} lines (scheduled May 2026)${NC}"
else
    echo -e "${GREEN}✅ REMOVED${NC}"
fi

# 5. Legacy Code
echo -n "Legacy Code:         "
if [ -d "code/crates/nestgate-core/src/data_sources/legacy" ]; then
    LEGACY_FILES=$(find code/crates/nestgate-core/src/data_sources/legacy -name "*.rs" | wc -l)
    echo -e "${RED}⚠️  ${LEGACY_FILES} files (action needed)${NC}"
else
    echo -e "${GREEN}✅ REMOVED${NC}"
fi

echo ""
echo "📈 UNIFICATION METRICS"
echo "───────────────────────────────────────────────"

# 6. TODO/FIXME Count
TODO_COUNT=$(grep -r "TODO\|FIXME\|HACK\|XXX" code/crates/ --include="*.rs" 2>/dev/null | wc -l || echo "0")
echo -n "Technical Debt:      "
if [ "$TODO_COUNT" -lt 10 ]; then
    echo -e "${GREEN}✅ ${TODO_COUNT} markers (excellent)${NC}"
elif [ "$TODO_COUNT" -lt 50 ]; then
    echo -e "${YELLOW}🟡 ${TODO_COUNT} markers (good)${NC}"
else
    echo -e "${RED}⚠️  ${TODO_COUNT} markers${NC}"
fi

# 7. async_trait Usage
ASYNC_TRAIT_COUNT=$(grep -r "#\[async_trait\]" code/crates/ --include="*.rs" 2>/dev/null | wc -l || echo "0")
echo -n "async_trait Usage:   "
if [ "$ASYNC_TRAIT_COUNT" -lt 25 ]; then
    PERCENT=$((100 - (ASYNC_TRAIT_COUNT * 100 / 116)))
    echo -e "${GREEN}✅ ${ASYNC_TRAIT_COUNT} instances (${PERCENT}% eliminated)${NC}"
else
    echo -e "${YELLOW}🟡 ${ASYNC_TRAIT_COUNT} instances${NC}"
fi

# 8. Arc<dyn> Patterns
ARC_DYN_COUNT=$(grep -r "Arc<dyn" code/crates/ --include="*.rs" 2>/dev/null | wc -l || echo "0")
echo -n "Arc<dyn> Patterns:   "
if [ "$ARC_DYN_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✅ 0 (100% eliminated)${NC}"
else
    echo -e "${YELLOW}🟡 ${ARC_DYN_COUNT} instances${NC}"
fi

# 9. Unified Directories
UNIFIED_DIRS=$(find code/crates -type d -name "*unified*" 2>/dev/null | wc -l || echo "0")
echo -n "Unified Directories: "
if [ "$UNIFIED_DIRS" -le 2 ]; then
    echo -e "${GREEN}✅ ${UNIFIED_DIRS} (consolidated)${NC}"
else
    echo -e "${YELLOW}🟡 ${UNIFIED_DIRS} (could consolidate)${NC}"
fi

echo ""
echo "🎯 UNIFICATION PROGRESS"
echo "───────────────────────────────────────────────"

# Calculate overall percentage
TOTAL_CHECKS=9
PASSED_CHECKS=0

# Count passing checks
[ "$(cargo check --workspace 2>&1 | grep -c "error" || echo "0")" -eq 0 ] && ((PASSED_CHECKS++))
[ "$OVERSIZED" -eq 0 ] && ((PASSED_CHECKS++))
[ "$TODO_COUNT" -lt 10 ] && ((PASSED_CHECKS++))
[ "$ASYNC_TRAIT_COUNT" -lt 25 ] && ((PASSED_CHECKS++))
[ "$ARC_DYN_COUNT" -eq 0 ] && ((PASSED_CHECKS++))
[ "$UNIFIED_DIRS" -le 2 ] && ((PASSED_CHECKS++))

PERCENTAGE=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))

echo -n "Overall Progress:    "
if [ "$PERCENTAGE" -ge 90 ]; then
    echo -e "${GREEN}${PERCENTAGE}% ✅ EXCELLENT${NC}"
elif [ "$PERCENTAGE" -ge 75 ]; then
    echo -e "${YELLOW}${PERCENTAGE}% 🟡 GOOD${NC}"
else
    echo -e "${RED}${PERCENTAGE}% ⚠️  NEEDS WORK${NC}"
fi

# Progress bar
BLOCKS=$((PERCENTAGE / 4))
REMAINING=$((25 - BLOCKS))
echo -n "["
printf "${GREEN}%0.s█${NC}" $(seq 1 $BLOCKS)
printf "%0.s░" $(seq 1 $REMAINING)
echo "] ${PERCENTAGE}%"

echo ""
echo "🚀 IMMEDIATE ACTIONS"
echo "───────────────────────────────────────────────"

# Check for immediate actions needed
ACTIONS_NEEDED=0

if [ -d "code/crates/nestgate-core/src/data_sources/legacy" ]; then
    echo -e "${YELLOW}⚠️  Remove or fix legacy data sources${NC}"
    echo "   → code/crates/nestgate-core/src/data_sources/legacy/"
    ((ACTIONS_NEEDED++))
fi

if cargo check --workspace 2>&1 | grep -q "error"; then
    echo -e "${RED}❌ Fix build errors${NC}"
    ((ACTIONS_NEEDED++))
fi

if [ "$OVERSIZED" -gt 0 ]; then
    echo -e "${RED}❌ Split ${OVERSIZED} oversized files${NC}"
    ((ACTIONS_NEEDED++))
fi

if [ "$ACTIONS_NEEDED" -eq 0 ]; then
    echo -e "${GREEN}✅ No immediate actions needed!${NC}"
    echo "   Codebase is production ready."
fi

echo ""
echo "📚 DOCUMENTATION"
echo "───────────────────────────────────────────────"
echo "Quick Start:     START_HERE_NOV_8_2025.md"
echo "Full Analysis:   UNIFICATION_COMPREHENSIVE_ANALYSIS_NOV_8_2025.md"
echo "Quick Actions:   UNIFICATION_QUICK_ACTIONS_NOV_8.md"
echo "Master Status:   PROJECT_STATUS_MASTER.md"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$PERCENTAGE" -ge 95 ]; then
    echo -e "${GREEN}   🎉 EXCELLENT STATUS - PRODUCTION READY! 🎉${NC}"
elif [ "$PERCENTAGE" -ge 85 ]; then
    echo -e "${YELLOW}   🟡 GOOD STATUS - MINOR POLISH NEEDED${NC}"
else
    echo -e "${RED}   ⚠️  ACTION REQUIRED - REVIEW NEEDED${NC}"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

exit 0

