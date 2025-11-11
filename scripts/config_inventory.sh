#!/bin/bash
# Configuration Struct Inventory Script
# Phase 2 Unification - Week 1, Day 1
# November 11, 2025

set -e

echo "========================================"
echo "Configuration Struct Inventory"
echo "Phase 2 Unification - Week 1, Day 1"
echo "========================================"
echo ""

# Create analysis directory if it doesn't exist
mkdir -p analysis

echo "Step 1: Finding all Config struct definitions..."
grep -r "pub struct.*Config" --include="*.rs" code/crates/ > analysis/config_structs.txt 2>/dev/null || true
grep -r "struct.*Config\|Configuration" --include="*.rs" code/crates/ >> analysis/config_structs.txt 2>/dev/null || true

TOTAL_CONFIGS=$(cat analysis/config_structs.txt | wc -l)
echo "✅ Found $TOTAL_CONFIGS Config struct definitions"
echo ""

echo "Step 2: Grouping by domain..."
echo "=== Network Configs ===" > analysis/config_by_domain.txt
grep -i "network" analysis/config_structs.txt >> analysis/config_by_domain.txt || echo "  (none found)" >> analysis/config_by_domain.txt
NETWORK_COUNT=$(grep -i "network" analysis/config_structs.txt | wc -l)
echo "  Network configs: $NETWORK_COUNT"

echo "" >> analysis/config_by_domain.txt
echo "=== Storage Configs ===" >> analysis/config_by_domain.txt
grep -i "storage\|zfs\|filesystem\|nas" analysis/config_structs.txt >> analysis/config_by_domain.txt || echo "  (none found)" >> analysis/config_by_domain.txt
STORAGE_COUNT=$(grep -i "storage\|zfs\|filesystem" analysis/config_structs.txt | wc -l)
echo "  Storage configs: $STORAGE_COUNT"

echo "" >> analysis/config_by_domain.txt
echo "=== Security Configs ===" >> analysis/config_by_domain.txt
grep -i "security\|auth\|tls\|encrypt\|ssl" analysis/config_structs.txt >> analysis/config_by_domain.txt || echo "  (none found)" >> analysis/config_by_domain.txt
SECURITY_COUNT=$(grep -i "security\|auth\|tls\|encrypt" analysis/config_structs.txt | wc -l)
echo "  Security configs: $SECURITY_COUNT"

echo "" >> analysis/config_by_domain.txt
echo "=== Handler/API Configs ===" >> analysis/config_by_domain.txt
grep -i "handler\|api\|rest\|rpc" analysis/config_structs.txt >> analysis/config_by_domain.txt || echo "  (none found)" >> analysis/config_by_domain.txt
HANDLER_COUNT=$(grep -i "handler\|api\|rest\|rpc" analysis/config_structs.txt | wc -l)
echo "  Handler/API configs: $HANDLER_COUNT"

echo ""
echo "Step 3: Finding duplicate config names..."
cat analysis/config_structs.txt | \
    sed 's/.*struct \([A-Za-z0-9_]*\).*/\1/' | \
    sort | uniq -c | sort -rn > analysis/config_duplicates.txt
echo "✅ Duplicate analysis complete"
echo ""

echo "Step 4: Finding canonical configs (already exist)..."
grep -r "canonical_primary" --include="*.rs" code/crates/nestgate-core/src/config/ | \
    grep "pub struct" > analysis/canonical_configs.txt 2>/dev/null || true
CANONICAL_COUNT=$(cat analysis/canonical_configs.txt | wc -l)
echo "  Existing canonical configs: $CANONICAL_COUNT"
echo ""

echo "========================================"
echo "INVENTORY COMPLETE!"
echo "========================================"
echo ""
echo "Results saved to analysis/ directory:"
echo "  - config_structs.txt       (all configs found)"
echo "  - config_by_domain.txt     (grouped by domain)"
echo "  - config_duplicates.txt    (duplicate analysis)"
echo "  - canonical_configs.txt    (existing canonical)"
echo ""
echo "Summary:"
echo "  Total configs found:     $TOTAL_CONFIGS"
echo "  Network configs:         $NETWORK_COUNT"
echo "  Storage configs:         $STORAGE_COUNT"
echo "  Security configs:        $SECURITY_COUNT"
echo "  Handler/API configs:     $HANDLER_COUNT"
echo "  Existing canonical:      $CANONICAL_COUNT"
echo ""
echo "Next: Review analysis/config_by_domain.txt for consolidation opportunities"

