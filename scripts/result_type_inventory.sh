#!/bin/bash
# Result Type Inventory Script
# Phase 2 Unification - Week 1, Day 1

set -e

echo "========================================"
echo "Result Type Inventory"
echo "Phase 2 Unification - Week 1, Day 1"
echo "========================================"
echo ""

mkdir -p analysis

echo "Step 1: Finding all Result type aliases..."
grep -r "pub type.*Result.*=.*Result\|type.*Result.*=.*Result" --include="*.rs" code/crates/ > analysis/result_types.txt 2>/dev/null || true
TOTAL_RESULTS=$(cat analysis/result_types.txt | wc -l)
echo "✅ Found $TOTAL_RESULTS Result type definitions"
echo ""

echo "Step 2: Grouping by domain..."
echo "=== Network Results ===" > analysis/result_types_by_domain.txt
grep -i "network" analysis/result_types.txt >> analysis/result_types_by_domain.txt || echo "  (none)" >> analysis/result_types_by_domain.txt
NETWORK_RESULTS=$(grep -i "network" analysis/result_types.txt | wc -l)
echo "  Network results: $NETWORK_RESULTS"

echo "" >> analysis/result_types_by_domain.txt
echo "=== Storage Results ===" >> analysis/result_types_by_domain.txt
grep -i "storage\|zfs" analysis/result_types.txt >> analysis/result_types_by_domain.txt || echo "  (none)" >> analysis/result_types_by_domain.txt
STORAGE_RESULTS=$(grep -i "storage\|zfs" analysis/result_types.txt | wc -l)
echo "  Storage results: $STORAGE_RESULTS"

echo "" >> analysis/result_types_by_domain.txt
echo "=== API/Handler Results ===" >> analysis/result_types_by_domain.txt
grep -i "api\|handler" analysis/result_types.txt >> analysis/result_types_by_domain.txt || echo "  (none)" >> analysis/result_types_by_domain.txt
API_RESULTS=$(grep -i "api\|handler" analysis/result_types.txt | wc -l)
echo "  API/Handler results: $API_RESULTS"

echo ""
echo "Step 3: Finding canonical Result (should exist)..."
grep -r "pub type Result<T>" --include="*.rs" code/crates/nestgate-core/src/error/ > analysis/canonical_result.txt 2>/dev/null || true
CANONICAL_EXISTS=$(cat analysis/canonical_result.txt | wc -l)
echo "  Canonical Result found: $CANONICAL_EXISTS"
echo ""

echo "========================================"
echo "RESULT TYPE INVENTORY COMPLETE!"
echo "========================================"
echo ""
echo "Summary:"
echo "  Total Result types:      $TOTAL_RESULTS"
echo "  Network results:         $NETWORK_RESULTS"
echo "  Storage results:         $STORAGE_RESULTS"
echo "  API/Handler results:     $API_RESULTS"
echo "  Canonical Result exists: $CANONICAL_EXISTS"
echo ""

