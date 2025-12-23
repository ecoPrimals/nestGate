#!/usr/bin/env bash
# NestGate Showcase - Master Runner
# Runs all demonstration levels with validation and receipts
set -euo pipefail

SHOWCASE_VERSION="1.0.0"
START_TIME=$(date +%s)
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RUN_ID="showcase-run-${TIMESTAMP}"
OUTPUT_BASE="outputs"
RUN_DIR="${OUTPUT_BASE}/${RUN_ID}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Create output directory
mkdir -p "$RUN_DIR/logs"
mkdir -p "$RUN_DIR/receipts"
mkdir -p "$RUN_DIR/demo_outputs"

# Logging
LOG_FILE="${RUN_DIR}/showcase.log"
exec > >(tee -a "$LOG_FILE") 2>&1

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}🎬 NestGate Showcase - Master Runner v${SHOWCASE_VERSION}${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}Run ID:${NC} ${RUN_ID}"
echo -e "${BLUE}Date:${NC} $(date)"
echo -e "${BLUE}Output:${NC} ${RUN_DIR}"
echo ""

# ============================================================================
# PHASE 1: PREREQUISITES CHECK
# ============================================================================

echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${MAGENTA}📋 Phase 1: Prerequisites Check${NC}"
echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

PREREQ_PASSED=true

# Check disk space
echo -n "💾 Checking disk space... "
AVAILABLE_GB=$(df -BG . | awk 'NR==2 {print $4}' | sed 's/G//')
if [ "$AVAILABLE_GB" -lt 10 ]; then
    echo -e "${RED}FAILED${NC} (Need 10GB, have ${AVAILABLE_GB}GB)"
    PREREQ_PASSED=false
else
    echo -e "${GREEN}OK${NC} (${AVAILABLE_GB}GB available)"
fi

# Check Rust
echo -n "🦀 Checking Rust installation... "
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    echo -e "${GREEN}OK${NC} (${RUST_VERSION})"
else
    echo -e "${RED}FAILED${NC} (cargo not found)"
    PREREQ_PASSED=false
fi

# Check ZFS (optional)
echo -n "📦 Checking ZFS... "
if command -v zpool &> /dev/null; then
    ZFS_VERSION=$(zpool --version | head -1 | awk '{print $2}')
    echo -e "${GREEN}OK${NC} (${ZFS_VERSION})"
    ZFS_AVAILABLE=true
else
    echo -e "${YELLOW}OPTIONAL${NC} (Will use filesystem backend)"
    ZFS_AVAILABLE=false
fi

# Check NestGate binary
echo -n "🏰 Checking NestGate build... "
cd ../..
if [ -f "target/release/nestgate" ] || [ -f "target/debug/nestgate" ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -n "Building... "
    if cargo build --release --quiet 2>&1 | tee "${RUN_DIR}/logs/build.log" > /dev/null; then
        echo -e "${GREEN}OK${NC}"
    else
        echo -e "${RED}FAILED${NC}"
        PREREQ_PASSED=false
    fi
fi
cd showcase

echo ""

if [ "$PREREQ_PASSED" = false ]; then
    echo -e "${RED}❌ Prerequisites check failed. Please resolve issues above.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ All prerequisites met!${NC}"
echo ""

# ============================================================================
# PHASE 2: SAFETY CHECKS
# ============================================================================

echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${MAGENTA}🛡️  Phase 2: Safety Checks${NC}"
echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Check for existing demo pools
if [ "$ZFS_AVAILABLE" = true ]; then
    echo -n "🔍 Checking for existing demo pools... "
    DEMO_POOLS=$(sudo zpool list 2>/dev/null | grep -c "nestgate-demo" || true)
    if [ "$DEMO_POOLS" -gt 0 ]; then
        echo -e "${YELLOW}WARNING${NC} (${DEMO_POOLS} demo pools exist)"
        echo "   Run cleanup script first: ./scripts/cleanup.sh"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    else
        echo -e "${GREEN}OK${NC}"
    fi
fi

# Check for running NestGate instances
echo -n "🔍 Checking for running NestGate instances... "
RUNNING_INSTANCES=$(pgrep -f "nestgate.*server" | wc -l || true)
if [ "$RUNNING_INSTANCES" -gt 0 ]; then
    echo -e "${YELLOW}WARNING${NC} (${RUNNING_INSTANCES} instances running)"
    echo "   Ports may conflict. Consider stopping them first."
else
    echo -e "${GREEN}OK${NC}"
fi

echo ""
echo -e "${GREEN}✅ Safety checks passed!${NC}"
echo ""

# ============================================================================
# PHASE 3: LEVEL 1 - ISOLATED INSTANCE
# ============================================================================

echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${MAGENTA}🎯 Phase 3: Level 1 - Isolated Instance (5 demos)${NC}"
echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

LEVEL1_DEMOS=(
    "01_isolated/01_storage_basics"
    "01_isolated/02_data_services"
    "01_isolated/03_capability_discovery"
    "01_isolated/04_health_monitoring"
    "01_isolated/05_zfs_advanced"
)

LEVEL1_PASSED=0
LEVEL1_FAILED=0
LEVEL1_SKIPPED=0

for i in "${!LEVEL1_DEMOS[@]}"; do
    DEMO="${LEVEL1_DEMOS[$i]}"
    DEMO_NUM=$((i + 1))
    DEMO_NAME=$(basename "$DEMO")
    
    echo -e "${BLUE}[${DEMO_NUM}/5]${NC} Running ${DEMO_NAME}..."
    
    if [ ! -f "${DEMO}/demo.sh" ]; then
        echo -e "   ${YELLOW}⚠️  SKIPPED${NC} (demo.sh not found)"
        LEVEL1_SKIPPED=$((LEVEL1_SKIPPED + 1))
        continue
    fi
    
    # Run demo (pass output directory)
    DEMO_START=$(date +%s)
    DEMO_OUTPUT="${RUN_DIR}/logs/${DEMO_NAME}.log"
    DEMO_OUTPUT_DIR="${RUN_DIR}/demo_outputs/${DEMO_NAME}"
    
    if bash "${DEMO}/demo.sh" "$DEMO_OUTPUT_DIR" > "$DEMO_OUTPUT" 2>&1; then
        DEMO_END=$(date +%s)
        DEMO_DURATION=$((DEMO_END - DEMO_START))
        echo -e "   ${GREEN}✅ PASSED${NC} (${DEMO_DURATION}s)"
        LEVEL1_PASSED=$((LEVEL1_PASSED + 1))
        
        # Copy receipt if exists
        LATEST_OUTPUT=$(ls -td ${DEMO_OUTPUT_DIR}/* 2>/dev/null | head -1 || echo "")
        if [ -n "$LATEST_OUTPUT" ] && [ -f "${LATEST_OUTPUT}/RECEIPT.md" ]; then
            cp "${LATEST_OUTPUT}/RECEIPT.md" "${RUN_DIR}/receipts/${DEMO_NAME}.md"
            fi
        fi
    else
        echo -e "   ${RED}❌ FAILED${NC}"
        LEVEL1_FAILED=$((LEVEL1_FAILED + 1))
        echo "   Log: ${DEMO_OUTPUT}"
    fi
done

echo ""
echo -e "${BLUE}Level 1 Summary:${NC}"
echo -e "   Passed:  ${GREEN}${LEVEL1_PASSED}${NC}"
echo -e "   Failed:  ${RED}${LEVEL1_FAILED}${NC}"
echo -e "   Skipped: ${YELLOW}${LEVEL1_SKIPPED}${NC}"
echo ""

# ============================================================================
# PHASE 4: GENERATE SUMMARY
# ============================================================================

echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${MAGENTA}📊 Phase 4: Summary${NC}"
echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

END_TIME=$(date +%s)
TOTAL_DURATION=$((END_TIME - START_TIME))
TOTAL_TESTS=$((LEVEL1_PASSED + LEVEL1_FAILED + LEVEL1_SKIPPED))

echo "🎬 Showcase Run Complete!"
echo ""
echo "📊 Statistics:"
echo "   Total Duration: ${TOTAL_DURATION}s"
echo "   Tests Run: ${TOTAL_TESTS}"
echo "   Passed: ${LEVEL1_PASSED}"
echo "   Failed: ${LEVEL1_FAILED}"
echo "   Skipped: ${LEVEL1_SKIPPED}"
echo ""
echo "📁 Output:"
echo "   Directory: ${RUN_DIR}"
echo "   Log: ${LOG_FILE}"
echo "   Receipts: ${RUN_DIR}/receipts/"
echo ""

# Generate master receipt
cat > "${RUN_DIR}/SHOWCASE_RECEIPT.md" <<EOF
# NestGate Showcase Run Receipt

**Run ID**: ${RUN_ID}
**Date**: $(date)
**Duration**: ${TOTAL_DURATION}s
**Status**: $( [ "$LEVEL1_FAILED" -eq 0 ] && echo "✅ SUCCESS" || echo "⚠️  PARTIAL" )

---

## Summary

- **Total Tests**: ${TOTAL_TESTS}
- **Passed**: ${LEVEL1_PASSED} ✅
- **Failed**: ${LEVEL1_FAILED} ❌
- **Skipped**: ${LEVEL1_SKIPPED} ⚠️

---

## Level 1: Isolated Instance

$(for i in "${!LEVEL1_DEMOS[@]}"; do
    DEMO="${LEVEL1_DEMOS[$i]}"
    DEMO_NAME=$(basename "$DEMO")
    if [ -f "${RUN_DIR}/receipts/${DEMO_NAME}.md" ]; then
        echo "- ✅ ${DEMO_NAME}"
    else
        echo "- ❌ ${DEMO_NAME}"
    fi
done)

---

## Prerequisites

- **Disk Space**: ${AVAILABLE_GB}GB available
- **Rust Version**: ${RUST_VERSION}
- **ZFS**: $( [ "$ZFS_AVAILABLE" = true ] && echo "Available (${ZFS_VERSION})" || echo "Not available" )
- **NestGate**: Built

---

## Files Generated

\`\`\`
$(ls -lh "${RUN_DIR}" | tail -n +2)
\`\`\`

---

## Individual Receipts

$(for receipt in ${RUN_DIR}/receipts/*.md; do
    if [ -f "$receipt" ]; then
        echo "- [$(basename "$receipt" .md)](receipts/$(basename "$receipt"))"
    fi
done)

---

**Showcase Version**: ${SHOWCASE_VERSION}
**Generated**: $(date)
EOF

if [ "$LEVEL1_FAILED" -eq 0 ]; then
    echo -e "${GREEN}🎉 All demonstrations passed!${NC}"
    EXIT_CODE=0
else
    echo -e "${YELLOW}⚠️  Some demonstrations failed. Check logs for details.${NC}"
    EXIT_CODE=1
fi

echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}📋 Master Receipt: ${RUN_DIR}/SHOWCASE_RECEIPT.md${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

exit $EXIT_CODE

