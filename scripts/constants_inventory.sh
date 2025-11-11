#!/bin/bash
# Constants Inventory Script
# Phase 2 Unification - Week 1, Day 1

set -e

echo "========================================"
echo "Constants Inventory"
echo "Phase 2 Unification - Week 1, Day 1"
echo "========================================"
echo ""

mkdir -p analysis

echo "Step 1: Finding all const declarations..."
grep -r "pub const [A-Z_]\|const [A-Z_]" --include="*.rs" code/crates/ > analysis/constants_all.txt 2>/dev/null || true
TOTAL_CONSTANTS=$(cat analysis/constants_all.txt | wc -l)
echo "✅ Found $TOTAL_CONSTANTS const declarations"
echo ""

echo "Step 2: Finding organized constants (in constants/ modules)..."
grep -r "pub const" --include="*.rs" code/crates/nestgate-core/src/constants/ > analysis/constants_organized.txt 2>/dev/null || true
ORGANIZED=$(cat analysis/constants_organized.txt | wc -l)
SCATTERED=$((TOTAL_CONSTANTS - ORGANIZED))
echo "  Organized (in constants/): $ORGANIZED"
echo "  Scattered (elsewhere):     $SCATTERED"
echo ""

echo "Step 3: Finding timeout constants..."
grep -i "timeout\|duration::from_secs" analysis/constants_all.txt > analysis/constants_timeouts.txt 2>/dev/null || true
TIMEOUTS=$(cat analysis/constants_timeouts.txt | wc -l)
echo "  Timeout constants: $TIMEOUTS"

echo "Step 4: Finding buffer size constants..."
grep -i "buffer.*size\|8192\|4096\|65536\|131072" analysis/constants_all.txt > analysis/constants_buffers.txt 2>/dev/null || true
BUFFERS=$(cat analysis/constants_buffers.txt | wc -l)
echo "  Buffer constants: $BUFFERS"

echo "Step 5: Finding port/network constants..."
grep -i "port\|8080\|8443" analysis/constants_all.txt > analysis/constants_ports.txt 2>/dev/null || true
PORTS=$(cat analysis/constants_ports.txt | wc -l)
echo "  Port/network constants: $PORTS"

echo "Step 6: Finding limit constants (MAX_, MIN_)..."
grep -E "MAX_|MIN_|LIMIT_" analysis/constants_all.txt > analysis/constants_limits.txt 2>/dev/null || true
LIMITS=$(cat analysis/constants_limits.txt | wc -l)
echo "  Limit constants: $LIMITS"

echo ""
echo "========================================"
echo "CONSTANTS INVENTORY COMPLETE!"
echo "========================================"
echo ""
echo "Summary:"
echo "  Total constants:         $TOTAL_CONSTANTS"
echo "  Already organized:       $ORGANIZED"
echo "  Scattered (to organize): $SCATTERED"
echo "  Timeouts:                $TIMEOUTS"
echo "  Buffers:                 $BUFFERS"
echo "  Ports:                   $PORTS"
echo "  Limits:                  $LIMITS"
echo ""

