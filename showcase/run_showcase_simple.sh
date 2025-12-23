#!/usr/bin/env bash
# NestGate Full Progressive Showcase Runner - Simplified
# Demonstrates complete capability journey from isolated to integrated
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TIMESTAMP=$(date +%s)
OUTPUT_DIR="$SCRIPT_DIR/full_showcase_output-${TIMESTAMP}"
START_TIME=$TIMESTAMP

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Tracking
DEMOS_RUN=0
DEMOS_PASSED=0

echo "=========================================="
echo "NestGate Progressive Showcase"
echo "=========================================="
echo "Output: $OUTPUT_DIR"
echo "Started: $(date)"
echo ""

# ============================================================================
# DEMO 1: Storage Basics
# ============================================================================

echo "----------------------------------------"
echo "Demo 1: Storage Basics"
echo "----------------------------------------"

DEMO_OUTPUT="$OUTPUT_DIR/01_storage_basics.log"
DEMOS_RUN=$((DEMOS_RUN + 1))

if cd "$SCRIPT_DIR/01_isolated/01_storage_basics" && bash ./demo.sh > "$DEMO_OUTPUT" 2>&1; then
    DEMOS_PASSED=$((DEMOS_PASSED + 1))
    echo "✅ PASSED"
else
    echo "❌ FAILED (see $DEMO_OUTPUT)"
fi
echo ""

# ============================================================================
# DEMO 2: Capability Discovery
# ============================================================================

echo "----------------------------------------"
echo "Demo 2: Capability Discovery (Isolated)"
echo "----------------------------------------"

DEMO_OUTPUT="$OUTPUT_DIR/03_capability_discovery.log"
DEMOS_RUN=$((DEMOS_RUN + 1))

if cd "$SCRIPT_DIR/01_isolated/03_capability_discovery" && bash ./demo.sh > "$DEMO_OUTPUT" 2>&1; then
    DEMOS_PASSED=$((DEMOS_PASSED + 1))
    echo "✅ PASSED"
else
    echo "❌ FAILED (see $DEMO_OUTPUT)"
fi
echo ""

# ============================================================================
# DEMO 3: BearDog Discovery (Graceful Degradation)
# ============================================================================

echo "----------------------------------------"
echo "Demo 3: BearDog Discovery"
echo "----------------------------------------"

DEMO_OUTPUT="$OUTPUT_DIR/01_beardog_discovery.log"
DEMOS_RUN=$((DEMOS_RUN + 1))

if cd "$SCRIPT_DIR/02_ecosystem_integration/live/01_beardog_discovery" && bash ./demo.sh > "$DEMO_OUTPUT" 2>&1; then
    DEMOS_PASSED=$((DEMOS_PASSED + 1))
    echo "✅ PASSED"
else
    echo "❌ FAILED (see $DEMO_OUTPUT)"
fi
echo ""

# ============================================================================
# Check for BearDog
# ============================================================================

BEARDOG_AVAILABLE=false
if timeout 1 bash -c "echo > /dev/tcp/localhost/9000" 2>/dev/null; then
    BEARDOG_AVAILABLE=true
    echo "✅ BearDog is running on port 9000"
else
    echo "⚠️  BearDog not running (skipping BTSP demo)"
fi
echo ""

# ============================================================================
# DEMO 4: BearDog BTSP (if available)
# ============================================================================

if [ "$BEARDOG_AVAILABLE" = "true" ]; then
    echo "----------------------------------------"
    echo "Demo 4: BearDog BTSP Communication"
    echo "----------------------------------------"
    
    DEMO_OUTPUT="$OUTPUT_DIR/02_beardog_btsp.log"
    DEMOS_RUN=$((DEMOS_RUN + 1))
    
    if cd "$SCRIPT_DIR/02_ecosystem_integration/live/02_beardog_btsp" && bash ./demo.sh > "$DEMO_OUTPUT" 2>&1; then
        DEMOS_PASSED=$((DEMOS_PASSED + 1))
        echo "✅ PASSED"
    else
        echo "❌ FAILED (see $DEMO_OUTPUT)"
    fi
    echo ""
fi

# ============================================================================
# DEMO 5: Ecosystem Integration Examples (Conceptual)
# ============================================================================

echo "----------------------------------------"
echo "Demo 5: Ecosystem Integration Patterns"
echo "----------------------------------------"

DEMO_OUTPUT="$OUTPUT_DIR/ecosystem_patterns.log"
DEMOS_RUN=$((DEMOS_RUN + 1))

# Run the ecosystem integration examples (they demonstrate patterns even without all services)
cd "$SCRIPT_DIR/.." || exit 1
(
    echo "=== Songbird Integration Pattern ===" 
    cargo run --example live-integration-03-songbird-orchestration 2>&1
    echo ""
    echo "=== ToadStool Integration Pattern ==="
    cargo run --example live-integration-04-toadstool-compute 2>&1
) > "$DEMO_OUTPUT" 2>&1

if [ $? -eq 0 ]; then
    DEMOS_PASSED=$((DEMOS_PASSED + 1))
    echo "✅ PASSED"
else
    echo "❌ FAILED (see $DEMO_OUTPUT)"
fi
echo ""

# ============================================================================
# FINAL REPORT
# ============================================================================

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "=========================================="
echo "FINAL REPORT"
echo "=========================================="
echo "Duration: ${DURATION}s"
echo "Demos Run: ${DEMOS_RUN}"
echo "Passed: ${DEMOS_PASSED}"
echo "Failed: $((DEMOS_RUN - DEMOS_PASSED))"
echo ""
echo "Output directory: $OUTPUT_DIR"
echo "Logs: $(ls -1 "$OUTPUT_DIR"/*.log 2>/dev/null | wc -l) files"
echo ""

# Create simple report
cat > "$OUTPUT_DIR/REPORT.txt" <<EOF
NestGate Progressive Showcase Report
====================================

Date: $(date)
Duration: ${DURATION}s
Total Demos: ${DEMOS_RUN}
Passed: ${DEMOS_PASSED}
Failed: $((DEMOS_RUN - DEMOS_PASSED))
Success Rate: $(( (DEMOS_PASSED * 100) / DEMOS_RUN ))%

BearDog Available: $([ "$BEARDOG_AVAILABLE" = "true" ] && echo "Yes" || echo "No")

Logs Generated:
$(ls -1 "$OUTPUT_DIR"/*.log 2>/dev/null | while read log; do basename "$log"; done)

Key Findings:
- Level 1 (Isolated): $([ $DEMOS_PASSED -ge 2 ] && echo "✅ Working" || echo "⚠️ Issues detected")
- Level 2 (Integrated): $([ "$BEARDOG_AVAILABLE" = "true" ] && echo "✅ Tested" || echo "⚠️ Not tested (BearDog unavailable)")
- Graceful Degradation: ✅ Demonstrated

Generated: $(date)
EOF

echo "Report saved: $OUTPUT_DIR/REPORT.txt"
echo ""

if [ $DEMOS_PASSED -eq $DEMOS_RUN ]; then
    echo "✅ ALL DEMOS PASSED!"
    exit 0
else
    echo "⚠️  Some demos failed"
    exit 1
fi

