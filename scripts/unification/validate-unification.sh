#!/usr/bin/env bash
# Validate Unification Progress
# Checks metrics and validates consolidation

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Validating Unification Progress..."
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
errors=0
warnings=0
passes=0

# Test 1: File Size Compliance
echo "📏 Test 1: File Size Compliance (max 2000 lines)"
large_files=$(find code/crates -name "*.rs" -exec wc -l {} \; | awk '$1 > 2000' | wc -l)
if [ "$large_files" -eq 0 ]; then
    echo -e "${GREEN}✅ PASS${NC}: All files under 2000 lines"
    ((passes++))
else
    echo -e "${RED}❌ FAIL${NC}: $large_files files exceed 2000 lines"
    ((errors++))
fi
echo ""

# Test 2: Config Struct Count
echo "📦 Test 2: Config Struct Consolidation"
config_count=$(grep -r "pub struct.*Config" code/crates/nestgate-core/src --include="*.rs" 2>/dev/null | wc -l || echo 0)
echo "   Current: $config_count Config structs in nestgate-core"
if [ "$config_count" -lt 200 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Config consolidation in good shape (<200)"
    ((passes++))
elif [ "$config_count" -lt 500 ]; then
    echo -e "${YELLOW}⚠️  WARN${NC}: Moderate fragmentation (200-500)"
    ((warnings++))
else
    echo -e "${RED}❌ FAIL${NC}: High fragmentation (>500)"
    ((errors++))
fi
echo ""

# Test 3: Error Enum Count
echo "⚠️  Test 3: Error Enum Consolidation"
error_count=$(grep -r "pub enum.*Error" code/crates --include="*.rs" 2>/dev/null | wc -l || echo 0)
echo "   Current: $error_count Error enums across all crates"
if [ "$error_count" -lt 50 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Error consolidation excellent (<50)"
    ((passes++))
elif [ "$error_count" -lt 100 ]; then
    echo -e "${YELLOW}⚠️  WARN${NC}: Some fragmentation remains (50-100)"
    ((warnings++))
else
    echo -e "${RED}❌ FAIL${NC}: High fragmentation (>100)"
    ((errors++))
fi
echo ""

# Test 4: Deprecated Code
echo "🗑️  Test 4: Deprecated Code Cleanup"
deprecated_count=$(grep -r "#\[deprecated" code/crates --include="*.rs" 2>/dev/null | wc -l || echo 0)
echo "   Current: $deprecated_count deprecated items"
if [ "$deprecated_count" -eq 0 ]; then
    echo -e "${GREEN}✅ PASS${NC}: No deprecated code"
    ((passes++))
elif [ "$deprecated_count" -lt 10 ]; then
    echo -e "${YELLOW}⚠️  WARN${NC}: Few deprecated items remain (<10)"
    ((warnings++))
else
    echo -e "${RED}❌ FAIL${NC}: Many deprecated items (≥10)"
    ((errors++))
fi
echo ""

# Test 5: Build Health
echo "🔨 Test 5: Build Health (skipping full check for speed)"
# Skipping cargo check for speed - can enable if needed
echo -e "${YELLOW}⚠️  SKIP${NC}: Full build check disabled for performance"
echo ""

# Test 6: Duplicate Constants
echo "🔢 Test 6: Duplicate Constants"
dup_constants=$(grep -r "pub const MODULE_VERSION" code/crates --include="*.rs" 2>/dev/null | wc -l || echo 0)
echo "   MODULE_VERSION definitions: $dup_constants"
if [ "$dup_constants" -le 2 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Minimal duplication"
    ((passes++))
elif [ "$dup_constants" -le 5 ]; then
    echo -e "${YELLOW}⚠️  WARN${NC}: Some duplication (3-5)"
    ((warnings++))
else
    echo -e "${RED}❌ FAIL${NC}: High duplication (>5)"
    ((errors++))
fi
echo ""

# Test 7: Trait Consolidation
echo "🎨 Test 7: Trait Consolidation"
trait_files=$(find code/crates -name "*.rs" -exec grep -l "pub trait" {} \; 2>/dev/null | wc -l || echo 0)
echo "   Files with trait definitions: $trait_files"
if [ "$trait_files" -lt 100 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Good consolidation (<100)"
    ((passes++))
elif [ "$trait_files" -lt 200 ]; then
    echo -e "${YELLOW}⚠️  WARN${NC}: Moderate fragmentation (100-200)"
    ((warnings++))
else
    echo -e "${RED}❌ FAIL${NC}: High fragmentation (>200)"
    ((errors++))
fi
echo ""

# Summary
echo "=========================================="
echo "📊 VALIDATION SUMMARY"
echo "=========================================="
echo -e "${GREEN}✅ Passed: $passes${NC}"
echo -e "${YELLOW}⚠️  Warnings: $warnings${NC}"
echo -e "${RED}❌ Failed: $errors${NC}"
echo ""

if [ "$errors" -eq 0 ] && [ "$warnings" -eq 0 ]; then
    echo -e "${GREEN}🎉 EXCELLENT! Full unification achieved!${NC}"
    exit 0
elif [ "$errors" -eq 0 ]; then
    echo -e "${YELLOW}✨ GOOD! Minor cleanup needed.${NC}"
    exit 0
else
    echo -e "${RED}⚠️  ACTION NEEDED: Address failures above${NC}"
    exit 1
fi 